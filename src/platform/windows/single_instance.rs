use crate::launch::{WINDOW_CLASS, WM_APP_REQUEST_EXIT_ID};

use super::error::Win32Error;
use windows::Win32::Foundation::{
    CloseHandle, ERROR_ALREADY_EXISTS, ERROR_INVALID_NAME, GetLastError, HANDLE, HWND, LPARAM,
    WPARAM,
};
use windows::Win32::System::Threading::CreateMutexW;
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowW, PostMessageW, SW_RESTORE, SetForegroundWindow, ShowWindow,
};
use windows::core::PCWSTR;

const PRODUCT_MUTEX_NAME: &str = r"Local\ShiftSpaceLangChange.SingleInstance";

/// Result of trying to become the process that owns the product mutex.
pub enum AcquireResult {
    Primary(SingleInstanceGuard),
    AlreadyRunning,
}

/// Owns the product mutex handle for exactly as long as the primary process runs.
pub struct SingleInstanceGuard {
    handle: HANDLE,
}

impl SingleInstanceGuard {
    pub fn acquire() -> Result<AcquireResult, Win32Error> {
        Self::acquire_named(PRODUCT_MUTEX_NAME)
    }

    pub fn acquire_named(name: &str) -> Result<AcquireResult, Win32Error> {
        if name.is_empty() || name.contains('\0') {
            return Err(Win32Error::new(ERROR_INVALID_NAME.0));
        }

        let name = wide(name);
        // Safety: the UTF-16 name is NUL-terminated and remains alive for this call. A null
        // security descriptor keeps the mutex in the current user's normal object namespace.
        let handle = unsafe { CreateMutexW(None, false, PCWSTR(name.as_ptr())) }
            .map_err(|error| Win32Error::new(error.code().0 as u32))?;

        // Safety: this reads the thread-local result immediately after CreateMutexW, before any
        // other Win32 call can change it. ERROR_ALREADY_EXISTS means this handle is a second
        // reference to the existing named mutex and must still be closed below.
        let already_exists = unsafe { GetLastError() } == ERROR_ALREADY_EXISTS;
        let guard = SingleInstanceGuard { handle };
        if already_exists {
            drop(guard);
            Ok(AcquireResult::AlreadyRunning)
        } else {
            Ok(AcquireResult::Primary(guard))
        }
    }
}

impl Drop for SingleInstanceGuard {
    fn drop(&mut self) {
        // Safety: handle came from a successful CreateMutexW call and is owned solely by this
        // guard. Drop runs once, so CloseHandle is called exactly once for this handle.
        let _ = unsafe { CloseHandle(self.handle) };
    }
}

/// Restore and focus the primary process's settings window when it exists.
pub fn show_existing_window() -> Result<bool, Win32Error> {
    let Some(hwnd) = find_existing_window() else {
        return Ok(false);
    };

    // Safety: hwnd was returned by FindWindowW and is passed only as a scalar window handle;
    // these calls do not borrow or retain any Rust pointer.
    let foregrounded = unsafe {
        let _ = ShowWindow(hwnd, SW_RESTORE);
        SetForegroundWindow(hwnd) == true
    };
    Ok(foregrounded)
}

/// Ask the primary process to terminate through its registered window message.
pub fn request_existing_exit() -> Result<bool, Win32Error> {
    let Some(hwnd) = find_existing_window() else {
        return Ok(false);
    };

    // Safety: hwnd is a live window handle returned by FindWindowW; the message carries only
    // zero scalar parameters and no pointer to Rust-owned memory.
    unsafe { PostMessageW(Some(hwnd), WM_APP_REQUEST_EXIT_ID, WPARAM(0), LPARAM(0)) }
        .map_err(|error| Win32Error::new(error.code().0 as u32))?;
    Ok(true)
}

fn find_existing_window() -> Option<HWND> {
    let class_name = wide(WINDOW_CLASS);
    // Safety: the class-name buffer is NUL-terminated and remains alive through the call; a null
    // window-name pointer asks Windows to match any title. FindWindowW returns no handle when
    // the primary window has not been created yet.
    unsafe { FindWindowW(PCWSTR(class_name.as_ptr()), PCWSTR(std::ptr::null())) }.ok()
}

fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}
