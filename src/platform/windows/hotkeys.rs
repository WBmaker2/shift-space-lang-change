use crate::config::Hotkey;
use crate::hotkeys::HotkeyBackend;
use crate::windows_mapping::hotkey_spec;
use windows::Win32::Foundation::{GetLastError, HWND};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, HOT_KEY_MODIFIERS, RegisterHotKey, UnregisterHotKey, VK_CONTROL, VK_SHIFT,
    VK_SPACE,
};

use super::error::Win32Error;

pub struct WinHotkeyBackend {
    hwnd: HWND,
}

impl WinHotkeyBackend {
    pub fn new(hwnd: HWND) -> Self {
        Self { hwnd }
    }
}

impl HotkeyBackend for WinHotkeyBackend {
    type Error = Win32Error;

    fn register(&mut self, hotkey: Hotkey) -> Result<(), Self::Error> {
        let spec = hotkey_spec(hotkey);
        // Safety: self.hwnd is owned by the app's message window and remains valid while the
        // backend is used; the scalar registration arguments are Win32-defined values.
        let result = unsafe {
            RegisterHotKey(
                Some(self.hwnd),
                spec.id,
                HOT_KEY_MODIFIERS(spec.modifiers),
                spec.virtual_key,
            )
        };
        result.map_err(|_| {
            // Safety: RegisterHotKey documents GetLastError as the failure source. This is the
            // first call after the failing API on this thread, so its thread-local raw Win32 code
            // is read before any other operation can overwrite it.
            Win32Error::new(unsafe { GetLastError().0 })
        })
    }

    fn unregister(&mut self, hotkey: Hotkey) -> Result<(), Self::Error> {
        let spec = hotkey_spec(hotkey);
        // Safety: the HWND and identifier are the same pair previously used for registration;
        // Windows does not retain a pointer to Rust memory during this call.
        let result = unsafe { UnregisterHotKey(Some(self.hwnd), spec.id) };
        result.map_err(|_| {
            // Safety: UnregisterHotKey documents GetLastError as the failure source. This is the
            // first call after the failing API on this thread, so its thread-local raw Win32 code
            // is read before any other operation can overwrite it.
            Win32Error::new(unsafe { GetLastError().0 })
        })
    }
}

pub fn keys_are_released() -> bool {
    is_released(VK_SHIFT) && is_released(VK_CONTROL) && is_released(VK_SPACE)
}

fn is_released(vkey: windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY) -> bool {
    // Safety: GetAsyncKeyState reads one scalar state for a valid virtual-key code and does not
    // dereference a caller-provided pointer.
    let state = unsafe { GetAsyncKeyState(vkey.0 as i32) };
    state & i16::MIN == 0
}
