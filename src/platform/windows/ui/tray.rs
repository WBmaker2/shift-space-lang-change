use crate::config::{AppSettings, Hotkey};
use crate::ui_model::{
    IDM_ABOUT, IDM_EXIT, IDM_SHOW, UiEvent, map_tray_callback_event, map_tray_command,
    tray_event_code,
};

use super::super::error::Win32Error;
use super::window::WM_APP_TRAY_CALLBACK;
use windows::Win32::Foundation::{GetLastError, HWND, LPARAM, POINT, RECT, WPARAM};
use windows::Win32::UI::Shell::{
    NIF_ICON, NIF_INFO, NIF_MESSAGE, NIF_SHOWTIP, NIF_TIP, NIIF_INFO, NIM_ADD, NIM_DELETE,
    NIM_MODIFY, NIM_SETFOCUS, NIM_SETVERSION, NOTIFYICON_VERSION_4, NOTIFYICONDATAW,
    Shell_NotifyIconW,
};
use windows::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreatePopupMenu, DestroyMenu, GetCursorPos, IDI_APPLICATION, MF_DISABLED,
    MF_SEPARATOR, MF_STRING, PostMessageW, SetForegroundWindow, TPM_RETURNCMD, TPM_RIGHTBUTTON,
    TrackPopupMenu, WM_CONTEXTMENU, WM_NULL, WM_RBUTTONUP,
};
use windows::core::{PCWSTR, w};

const TRAY_ICON_ID: u32 = 1;

/// Owns the shell notification icon and removes it when the app leaves the message loop.
pub struct TrayIcon {
    data: NOTIFYICONDATAW,
    installed: bool,
}

impl TrayIcon {
    pub fn install(hwnd: HWND) -> Result<Self, Win32Error> {
        let icon =
            unsafe { windows::Win32::UI::WindowsAndMessaging::LoadIconW(None, IDI_APPLICATION) }
                .map_err(win_error)?;
        let mut data = NOTIFYICONDATAW {
            cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uID: TRAY_ICON_ID,
            uFlags: NIF_MESSAGE | NIF_ICON | NIF_TIP | NIF_SHOWTIP,
            uCallbackMessage: WM_APP_TRAY_CALLBACK,
            hIcon: icon,
            ..Default::default()
        };
        write_tip(&mut data.szTip, "한/영 전환 도우미");
        // Safety: data is fully initialized, the icon handle is the shared system icon, and the
        // shell copies the notification data during this synchronous call.
        if !unsafe { Shell_NotifyIconW(NIM_ADD, &data) }.as_bool() {
            return Err(last_error());
        }
        data.Anonymous.uVersion = NOTIFYICON_VERSION_4;
        // Safety: the same notification record identifies the icon just added; no Rust pointer
        // is retained by the shell.
        if !unsafe { Shell_NotifyIconW(NIM_SETVERSION, &data) }.as_bool() {
            let error = last_error();
            // Best effort cleanup keeps a failed installation from leaving a stale icon.
            let _ = unsafe { Shell_NotifyIconW(NIM_DELETE, &data) };
            return Err(error);
        }
        Ok(Self {
            data,
            installed: true,
        })
    }

    /// Display a best-effort Windows toast without changing the installed icon registration.
    pub fn notify(&self, title: &str, body: &str) -> Result<(), Win32Error> {
        let mut data = self.data;
        data.uFlags = NIF_INFO;
        write_utf16_truncated(&mut data.szInfoTitle, title);
        write_utf16_truncated(&mut data.szInfo, body);
        data.dwInfoFlags = NIIF_INFO;
        // Safety: data identifies this installed icon and both fixed UTF-16 buffers are fully
        // initialized with a terminating NUL; the shell copies the notification synchronously.
        if unsafe { Shell_NotifyIconW(NIM_MODIFY, &data) }.as_bool() {
            Ok(())
        } else {
            Err(last_error())
        }
    }

    /// Handle the shell callback; right click opens a menu and double click shows the window.
    pub fn read_event(
        &self,
        _wparam: usize,
        lparam: isize,
        settings: AppSettings,
    ) -> Result<Option<UiEvent>, Win32Error> {
        let event = tray_event_code(lparam);
        if let Some(event) = map_tray_callback_event(lparam) {
            return Ok(Some(event));
        }
        if event == WM_RBUTTONUP || event == WM_CONTEXTMENU {
            return self.show_menu(settings);
        }
        Ok(None)
    }

    fn show_menu(&self, settings: AppSettings) -> Result<Option<UiEvent>, Win32Error> {
        let menu = unsafe { CreatePopupMenu() }.map_err(win_error)?;
        let summary = active_summary(settings);
        let summary_wide = wide(&summary);
        // Safety: menu is a newly created popup owned by this scope; all menu text buffers remain
        // alive through each synchronous AppendMenuW call.
        let result = unsafe {
            AppendMenuW(menu, MF_STRING, IDM_SHOW, w!("설정 열기"))
                .and_then(|()| {
                    AppendMenuW(
                        menu,
                        MF_STRING | MF_DISABLED,
                        0,
                        PCWSTR(summary_wide.as_ptr()),
                    )
                })
                .and_then(|()| AppendMenuW(menu, MF_STRING, IDM_ABOUT, w!("프로그램 정보")))
                .and_then(|()| AppendMenuW(menu, MF_SEPARATOR, 0, PCWSTR::null()))
        };
        if let Err(error) = result {
            let _ = unsafe { DestroyMenu(menu) };
            return Err(win_error(error));
        }
        let result = unsafe { AppendMenuW(menu, MF_STRING, IDM_EXIT, w!("종료")) };
        if let Err(error) = result {
            let _ = unsafe { DestroyMenu(menu) };
            return Err(win_error(error));
        }

        let mut point = POINT::default();
        // Safety: point is a writable local output buffer and menu belongs to this scope.
        let cursor_result = unsafe { GetCursorPos(&mut point) };
        if let Err(error) = cursor_result {
            let _ = unsafe { DestroyMenu(menu) };
            return Err(win_error(error));
        }
        // The foreground call is required by Windows so the popup closes when focus leaves it.
        let _ = unsafe { SetForegroundWindow(self.data.hWnd) };
        // Safety: menu and owner HWND are valid for this synchronous call; TPM_RETURNCMD returns
        // the selected command without posting a pointer-bearing message.
        let selected = unsafe {
            TrackPopupMenu(
                menu,
                TPM_RETURNCMD | TPM_RIGHTBUTTON,
                point.x,
                point.y,
                Some(0),
                self.data.hWnd,
                Some(std::ptr::null::<RECT>()),
            )
        };
        // Windows recommends a benign message after TrackPopupMenu so the foreground window
        // transition is completed and the popup is dismissed reliably when focus changes.
        // Safety: the owner handle is app-owned and the message carries no borrowed data.
        let _ = unsafe { Shell_NotifyIconW(NIM_SETFOCUS, &self.data) };
        let _ = unsafe { PostMessageW(Some(self.data.hWnd), WM_NULL, WPARAM(0), LPARAM(0)) };
        let _ = unsafe { DestroyMenu(menu) };
        if selected.0 == 0 {
            Ok(None)
        } else {
            Ok(map_tray_command(selected.0 as usize))
        }
    }
}

impl Drop for TrayIcon {
    fn drop(&mut self) {
        if self.installed {
            // Safety: data identifies the icon added by install and remains initialized until
            // this synchronous removal call. Drop is the sole owner of this shell registration.
            let _ = unsafe { Shell_NotifyIconW(NIM_DELETE, &self.data) };
            self.installed = false;
        }
    }
}

fn write_tip(destination: &mut [u16; 128], value: &str) {
    write_utf16_truncated(destination, value);
}

fn write_utf16_truncated(destination: &mut [u16], value: &str) {
    destination.fill(0);
    if destination.is_empty() {
        return;
    }
    let payload = destination.len() - 1;
    for (slot, unit) in destination[..payload].iter_mut().zip(value.encode_utf16()) {
        *slot = unit;
    }
    if payload > 0 && is_high_surrogate(destination[payload - 1]) {
        destination[payload - 1] = 0;
    }
}

fn is_high_surrogate(unit: u16) -> bool {
    (0xD800..=0xDBFF).contains(&unit)
}

fn active_summary(settings: AppSettings) -> String {
    let mut active = Vec::new();
    if settings.is_enabled(Hotkey::ShiftSpace) {
        active.push("Shift + Space");
    }
    if settings.is_enabled(Hotkey::CtrlSpace) {
        active.push("Ctrl + Space");
    }
    format!("활성 단축키: {}", active.join(", "))
}

fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}

fn win_error(error: windows::core::Error) -> Win32Error {
    Win32Error::new(error.code().0 as u32)
}

fn last_error() -> Win32Error {
    Win32Error::new(unsafe { GetLastError() }.0)
}

#[cfg(test)]
mod tests {
    use crate::config::AppSettings;

    use super::{active_summary, write_utf16_truncated};

    #[test]
    fn active_summary_lists_both_enabled_hotkeys() {
        assert_eq!(
            active_summary(AppSettings::default()),
            "활성 단축키: Shift + Space, Ctrl + Space"
        );
    }

    #[test]
    fn active_summary_lists_the_remaining_hotkey() {
        let settings = AppSettings::new(false, true).expect("one hotkey remains enabled");
        assert_eq!(active_summary(settings), "활성 단축키: Ctrl + Space");
    }

    #[test]
    fn notification_payload_is_nul_terminated_and_truncated() {
        let mut payload = [0_u16; 4];
        write_utf16_truncated(&mut payload, "가나다라마");
        assert_eq!(payload, ['가' as u16, '나' as u16, '다' as u16, 0]);
    }

    #[test]
    fn notification_payload_clears_old_content() {
        let mut payload = [9_u16; 4];
        write_utf16_truncated(&mut payload, "A");
        assert_eq!(payload, ['A' as u16, 0, 0, 0]);
    }

    #[test]
    fn notification_payload_does_not_split_surrogate_pairs() {
        let mut payload = [0_u16; 3];
        write_utf16_truncated(&mut payload, "A😀");
        assert_eq!(payload, ['A' as u16, 0, 0]);
    }
}
