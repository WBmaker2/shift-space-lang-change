use crate::config::AppSettings;
use crate::launch::{WINDOW_CLASS, WM_APP_REQUEST_EXIT_ID};
use crate::ui_model::{
    IDC_CTRL_SPACE, IDC_HIDE, IDC_SHIFT_SPACE, IDC_STARTUP, UiEvent, map_command_notification,
    map_queued_command, map_tray_callback_event,
};

use super::super::error::Win32Error;
use windows::Win32::Foundation::{
    ERROR_CLASS_ALREADY_EXISTS, GetLastError, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM,
};
use windows::Win32::Graphics::Gdi::{COLOR_WINDOW, HBRUSH};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY;
use windows::Win32::UI::WindowsAndMessaging::{
    BM_GETCHECK, BM_SETCHECK, BS_AUTOCHECKBOX, BS_PUSHBUTTON, CS_HREDRAW, CS_VREDRAW,
    CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, DestroyWindow, IDC_ARROW, IDI_APPLICATION,
    LoadCursorW, LoadIconW, MSG, PostMessageW, RegisterClassExW, SW_HIDE, WINDOW_EX_STYLE,
    WINDOW_STYLE, WM_APP, WM_CLOSE, WM_COMMAND, WM_KEYDOWN, WNDCLASSEXW, WS_CAPTION, WS_CHILD,
    WS_CLIPCHILDREN, WS_EX_CONTROLPARENT, WS_MINIMIZEBOX, WS_SYSMENU, WS_TABSTOP, WS_VISIBLE,
};
use windows::core::{PCWSTR, w};

/// Shell callback message registered in `NOTIFYICONDATAW.uCallbackMessage`.
///
/// The window procedure receives this message directly from Explorer, so it must be forwarded
/// to the app queue before the normal queue-based event handling can see it.
pub const WM_APP_TRAY_CALLBACK: u32 = WM_APP + 1;

/// Private queue message carrying the scalar payload from a shell tray callback.
/// `WM_APP + 2` is reserved for the cross-process exit request and `WM_APP + 3` for UI commands.
pub const WM_APP_TRAY_QUEUE: u32 = WM_APP + 4;

/// Private queued message carrying a button notification received synchronously by `window_proc`.
/// `WM_APP + 2` is already reserved for the cross-process exit request.
pub const WM_APP_UI_COMMAND: u32 = WM_APP + 3;

/// Handles for the window and each control that the controller needs to render.
#[derive(Clone, Copy, Debug)]
pub struct UiHandles {
    pub hwnd: HWND,
    pub shift_space: HWND,
    pub ctrl_space: HWND,
    pub startup: HWND,
    pub status: HWND,
    pub hide: HWND,
}

/// Create the small modeless settings window with only standard Win32 controls.
pub fn create_settings_window() -> Result<UiHandles, Win32Error> {
    let instance = module_instance()?;
    register_class(instance)?;
    let class_name = wide(WINDOW_CLASS);
    let title = wide("한/영 전환 도우미");
    // Safety: all strings and the module handle remain valid for the duration of the call, and
    // the null parent/parameter values request a top-level window with no borrowed Rust state.
    let hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE(WS_EX_CONTROLPARENT.0),
            PCWSTR(class_name.as_ptr()),
            PCWSTR(title.as_ptr()),
            WINDOW_STYLE(WS_CAPTION.0 | WS_SYSMENU.0 | WS_MINIMIZEBOX.0 | WS_CLIPCHILDREN.0),
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            420,
            300,
            None,
            None,
            Some(instance),
            None,
        )
    }
    .map_err(win_error)?;
    let window_guard = WindowGuard { hwnd };

    let status = create_control(
        hwnd,
        ControlSpec {
            class: w!("STATIC"),
            text: "실행 중",
            x: 16,
            y: 18,
            width: 375,
            height: 24,
            id: 0,
            tab_stop: false,
        },
    )?;
    let hint = create_control(
        hwnd,
        ControlSpec {
            class: w!("STATIC"),
            text: "선택한 단축키로 한/영 입력을 전환합니다.",
            x: 16,
            y: 48,
            width: 375,
            height: 22,
            id: 0,
            tab_stop: false,
        },
    )?;
    let shift_space = create_control(
        hwnd,
        ControlSpec {
            class: w!("BUTTON"),
            text: "Shift + Space",
            x: 22,
            y: 88,
            width: 350,
            height: 26,
            id: IDC_SHIFT_SPACE,
            tab_stop: true,
        },
    )?;
    let ctrl_space = create_control(
        hwnd,
        ControlSpec {
            class: w!("BUTTON"),
            text: "Ctrl + Space",
            x: 22,
            y: 120,
            width: 350,
            height: 26,
            id: IDC_CTRL_SPACE,
            tab_stop: true,
        },
    )?;
    let startup = create_control(
        hwnd,
        ControlSpec {
            class: w!("BUTTON"),
            text: "Windows 시작 시 자동 실행",
            x: 22,
            y: 170,
            width: 350,
            height: 26,
            id: IDC_STARTUP,
            tab_stop: true,
        },
    )?;
    let hide = create_control(
        hwnd,
        ControlSpec {
            class: w!("BUTTON"),
            text: "트레이로 숨기기",
            x: 270,
            y: 220,
            width: 120,
            height: 28,
            id: IDC_HIDE,
            tab_stop: true,
        },
    )?;
    let _ = hint;
    Ok(UiHandles {
        hwnd: window_guard.into_hwnd(),
        shift_space,
        ctrl_space,
        startup,
        status,
        hide,
    })
}

/// Reconcile every control with the controller's actual state after a successful or failed edit.
pub fn render_state(
    handles: &UiHandles,
    settings: AppSettings,
    startup_enabled: bool,
    status: &str,
) -> Result<(), Win32Error> {
    set_checked(handles.shift_space, settings.shift_space_enabled())?;
    set_checked(handles.ctrl_space, settings.ctrl_space_enabled())?;
    set_checked(handles.startup, startup_enabled)?;
    let status = wide(status);
    // Safety: the status window and UTF-16 text are valid for this call; Windows copies the text.
    unsafe {
        windows::Win32::UI::WindowsAndMessaging::SetWindowTextW(
            handles.status,
            PCWSTR(status.as_ptr()),
        )
    }
    .map_err(win_error)
}

/// Convert a queued Windows message into a platform-neutral UI event.
pub fn read_ui_event(message: &MSG) -> Option<UiEvent> {
    match message.message {
        WM_COMMAND => {
            let id = (message.wParam.0 & 0xffff) as i32;
            let checked = matches!(id, IDC_SHIFT_SPACE | IDC_CTRL_SPACE | IDC_STARTUP)
                && is_checked(HWND(message.lParam.0 as *mut _));
            map_command_notification(message.wParam.0, checked)
        }
        WM_APP_UI_COMMAND => map_queued_command(message.wParam.0),
        WM_CLOSE => Some(UiEvent::Hide),
        WM_KEYDOWN if message.wParam.0 == VIRTUAL_KEY(0x1b).0 as usize => Some(UiEvent::Hide),
        WM_APP_REQUEST_EXIT_ID => Some(UiEvent::Exit),
        WM_APP_TRAY_QUEUE => map_tray_callback_event(message.lParam.0),
        _ => None,
    }
}

struct WindowGuard {
    hwnd: HWND,
}

impl WindowGuard {
    fn into_hwnd(mut self) -> HWND {
        let hwnd = self.hwnd;
        self.hwnd = HWND::default();
        hwnd
    }
}

impl Drop for WindowGuard {
    fn drop(&mut self) {
        if !self.hwnd.0.is_null() {
            // Safety: the guard owns the top-level HWND created by create_settings_window; this
            // best-effort cleanup runs only on a failed child-control construction path.
            let _ = unsafe { DestroyWindow(self.hwnd) };
        }
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if message == WM_APP_TRAY_CALLBACK {
        // Explorer delivers the registered Shell callback directly to this window procedure.
        // Re-post only the scalar payload under a different private message so the app loop can
        // consume it without recursively routing the callback back through this branch.
        // Safety: hwnd is the live callback window and both message parameters are scalar values;
        // PostMessageW does not retain a Rust pointer.
        let _ = unsafe { PostMessageW(Some(hwnd), WM_APP_TRAY_QUEUE, wparam, lparam) };
        return LRESULT(0);
    }
    if message == WM_COMMAND && map_command_notification(wparam.0, false).is_some() {
        let id = (wparam.0 & 0xffff) as i32;
        let checked = matches!(id, IDC_SHIFT_SPACE | IDC_CTRL_SPACE | IDC_STARTUP)
            && is_checked(HWND(lparam.0 as *mut _));
        let posted_wparam = (wparam.0 & 0xffff) | (usize::from(checked) << 16);
        // WM_COMMAND is delivered synchronously to the parent and therefore never reaches the
        // queue read by message_loop. Re-post a compact scalar payload so the existing typed
        // event path can process it later, preserving checkbox state at click time.
        // Safety: hwnd is the live callback window and both message parameters are scalar values;
        // PostMessageW does not retain a Rust pointer.
        let _ = unsafe {
            PostMessageW(
                Some(hwnd),
                WM_APP_UI_COMMAND,
                WPARAM(posted_wparam),
                LPARAM(0),
            )
        };
        return LRESULT(0);
    }
    if message == WM_CLOSE {
        // Safety: hwnd is the callback's live window handle; hiding it preserves the tray app.
        unsafe {
            let _ = windows::Win32::UI::WindowsAndMessaging::ShowWindow(hwnd, SW_HIDE);
        };
        return LRESULT(0);
    }
    // Safety: no application-owned pointers are passed to the default window procedure.
    unsafe { DefWindowProcW(hwnd, message, wparam, lparam) }
}

fn module_instance() -> Result<HINSTANCE, Win32Error> {
    // Safety: a null module name requests the current executable module and no pointer is read.
    let module = unsafe { GetModuleHandleW(None) }.map_err(win_error)?;
    Ok(HINSTANCE(module.0))
}

fn register_class(instance: HINSTANCE) -> Result<(), Win32Error> {
    let class_name = wide(WINDOW_CLASS);
    let cursor = unsafe { LoadCursorW(None, IDC_ARROW) }.map_err(win_error)?;
    let icon = unsafe { LoadIconW(None, IDI_APPLICATION) }.map_err(win_error)?;
    let class = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(window_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: instance,
        hIcon: icon,
        hCursor: cursor,
        hbrBackground: HBRUSH((COLOR_WINDOW.0 + 1) as usize as *mut _),
        lpszMenuName: PCWSTR::null(),
        lpszClassName: PCWSTR(class_name.as_ptr()),
        hIconSm: icon,
    };
    // Safety: class points to initialized data and all referenced strings live through the call.
    let atom = unsafe { RegisterClassExW(&class) };
    if atom == 0 {
        let error = unsafe { GetLastError() };
        if error != ERROR_CLASS_ALREADY_EXISTS {
            return Err(Win32Error::new(error.0));
        }
    }
    Ok(())
}

struct ControlSpec<'a> {
    class: PCWSTR,
    text: &'a str,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    id: i32,
    tab_stop: bool,
}

fn create_control(parent: HWND, spec: ControlSpec<'_>) -> Result<HWND, Win32Error> {
    let ControlSpec {
        class,
        text,
        x,
        y,
        width,
        height,
        id,
        tab_stop,
    } = spec;
    let text = wide(text);
    let mut style = WS_CHILD.0 | WS_VISIBLE.0;
    if tab_stop {
        style |= WS_TABSTOP.0;
    }
    if id == IDC_HIDE {
        style |= BS_PUSHBUTTON as u32;
    } else if id != 0 {
        style |= BS_AUTOCHECKBOX as u32;
    }
    // Safety: parent is the newly created window; class/text buffers and the integer control ID
    // remain valid for this call, with no pointer retained by the wrapper.
    unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE(0),
            class,
            PCWSTR(text.as_ptr()),
            WINDOW_STYLE(style),
            x,
            y,
            width,
            height,
            Some(parent),
            Some(windows::Win32::UI::WindowsAndMessaging::HMENU(
                id as usize as *mut _,
            )),
            None,
            None,
        )
    }
    .map_err(win_error)
}

fn set_checked(hwnd: HWND, checked: bool) -> Result<(), Win32Error> {
    // Safety: hwnd is a child checkbox created by create_settings_window; this message carries
    // scalar state only and does not borrow a Rust buffer.
    unsafe {
        windows::Win32::UI::WindowsAndMessaging::SendMessageW(
            hwnd,
            BM_SETCHECK,
            Some(WPARAM(if checked { 1 } else { 0 })),
            Some(LPARAM(0)),
        );
    }
    Ok(())
}

fn is_checked(hwnd: HWND) -> bool {
    // Safety: hwnd comes from a WM_COMMAND notification and is a checkbox handle owned by this
    // window; only a scalar result is returned.
    unsafe {
        windows::Win32::UI::WindowsAndMessaging::SendMessageW(
            hwnd,
            BM_GETCHECK,
            Some(WPARAM(0)),
            Some(LPARAM(0)),
        )
        .0 == 1
    }
}

fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}

fn win_error(error: windows::core::Error) -> Win32Error {
    Win32Error::new(error.code().0 as u32)
}

#[cfg(test)]
mod tests {
    use super::{WM_APP_TRAY_CALLBACK, WM_APP_TRAY_QUEUE, WM_APP_UI_COMMAND, read_ui_event};
    use crate::launch::WM_APP_REQUEST_EXIT_ID;
    use crate::ui_model::UiEvent;
    use windows::Win32::Foundation::{LPARAM, WPARAM};
    use windows::Win32::UI::WindowsAndMessaging::{MSG, WM_LBUTTONDBLCLK};

    #[test]
    fn tray_callback_and_queue_messages_are_distinct_and_reserved() {
        assert_ne!(WM_APP_TRAY_CALLBACK, WM_APP_TRAY_QUEUE);
        assert_ne!(WM_APP_TRAY_QUEUE, WM_APP_REQUEST_EXIT_ID);
        assert_ne!(WM_APP_TRAY_QUEUE, WM_APP_UI_COMMAND);
    }

    #[test]
    fn shell_callback_is_not_consumed_as_an_app_event() {
        let message = MSG {
            message: WM_APP_TRAY_CALLBACK,
            wParam: WPARAM(1),
            lParam: LPARAM(WM_LBUTTONDBLCLK as isize),
            ..Default::default()
        };
        assert_eq!(read_ui_event(&message), None);
    }

    #[test]
    fn queued_tray_callback_is_converted_to_show_event() {
        let message = MSG {
            message: WM_APP_TRAY_QUEUE,
            wParam: WPARAM(1),
            lParam: LPARAM(WM_LBUTTONDBLCLK as isize),
            ..Default::default()
        };
        assert_eq!(read_ui_event(&message), Some(UiEvent::Show));
    }
}
