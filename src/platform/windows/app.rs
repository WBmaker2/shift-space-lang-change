use std::time::Instant;

use crate::config::{AppSettings, Hotkey};
use crate::controller::{AppController, ControllerError, ControllerEvent};
use crate::hotkeys::{ApplyError, HotkeyManager};
use crate::launch::LaunchMode;
use crate::platform::windows::ui::{
    TrayIcon, UiHandles, create_settings_window, read_ui_event, render_state,
};
use crate::platform::windows::{
    AcquireResult, RegistrySettingsStore, SingleInstanceGuard, WinHotkeyBackend, WinImeSender,
    WinStartupController, keys_are_released, request_existing_exit, show_existing_window,
};
use crate::ports::{SettingsStore, StartupController};
use crate::ui_model::UiEvent;
use crate::windows_mapping::hotkey_spec;

use super::error::Win32Error;
use super::timer::{TimerGuard, should_process_timer};
use super::ui::window::WM_APP_TRAY;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, IsDialogMessageW, MB_ICONERROR, MB_OK, MSG, MessageBoxW,
    PostQuitMessage, SW_HIDE, SW_SHOW, SetForegroundWindow, ShowWindow, TranslateMessage,
    WM_DESTROY, WM_HOTKEY, WM_TIMER,
};
use windows::core::PCWSTR;

type Controller =
    AppController<WinHotkeyBackend, RegistrySettingsStore, WinStartupController, WinImeSender>;

/// Run the native Windows process and return the desired process exit code.
pub fn run(mode: LaunchMode) -> Result<i32, Win32Error> {
    if mode == LaunchMode::QuitExisting {
        request_existing_exit()?;
        return Ok(0);
    }

    let instance = match SingleInstanceGuard::acquire()? {
        AcquireResult::Primary(guard) => guard,
        AcquireResult::AlreadyRunning => {
            show_existing_window()?;
            return Ok(0);
        }
    };

    let handles = create_settings_window()?;
    let settings_store = RegistrySettingsStore::new();
    let (hotkeys, initial_status) = match load_hotkeys(&settings_store, handles.hwnd) {
        Ok(value) => value,
        Err(HotkeyLoadError::Fatal(error)) => {
            destroy_window(handles.hwnd);
            drop(instance);
            return Err(error);
        }
        Err(HotkeyLoadError::BothConflict) => {
            destroy_window(handles.hwnd);
            drop(instance);
            show_error_message(
                "한/영 전환 도우미",
                "Shift + Space와 Ctrl + Space를 모두 등록할 수 없습니다.\n다른 프로그램의 단축키를 해제한 뒤 다시 실행해 주세요.",
            );
            return Ok(1);
        }
    };

    let executable = match std::env::current_exe() {
        Ok(path) => path,
        Err(_) => {
            destroy_window(handles.hwnd);
            drop(instance);
            return Err(Win32Error::new(1));
        }
    };
    let startup = WinStartupController::new(&executable);
    let startup_enabled = match startup.is_enabled() {
        Ok(enabled) => enabled,
        Err(error) => {
            destroy_window(handles.hwnd);
            drop(instance);
            return Err(error);
        }
    };
    let ime = WinImeSender;
    let mut controller = AppController::new(hotkeys, settings_store, startup, ime, startup_enabled);
    let mut tray = match TrayIcon::install(handles.hwnd) {
        Ok(tray) => tray,
        Err(error) => {
            destroy_window(handles.hwnd);
            drop(controller);
            drop(instance);
            return Err(error);
        }
    };

    render_state(
        &handles,
        controller.settings(),
        controller.startup_enabled(),
        &initial_status,
    )?;
    if mode == LaunchMode::Foreground {
        show_window(handles.hwnd);
    } else {
        hide_window(handles.hwnd);
    }

    if initial_status != "실행 중" {
        notify_best_effort(&mut tray, &initial_status);
    }
    let mut timer = TimerGuard::new(handles.hwnd);
    let exit_code = message_loop(&handles, &mut tray, &mut controller, &mut timer)?;

    drop(timer);
    drop(controller);
    drop(tray);
    destroy_window(handles.hwnd);
    drop(instance);
    Ok(exit_code)
}

fn load_hotkeys(
    store: &RegistrySettingsStore,
    hwnd: HWND,
) -> Result<(HotkeyManager<WinHotkeyBackend>, String), HotkeyLoadError> {
    let requested = store.load()?;
    match HotkeyManager::new(WinHotkeyBackend::new(hwnd), requested) {
        Ok(manager) => Ok((manager, "실행 중".to_owned())),
        Err(error) => {
            let conflict = match error {
                ApplyError::Register { hotkey, .. } => hotkey,
                ApplyError::Rollback { .. } | ApplyError::Unregister { .. } => {
                    return Err(HotkeyLoadError::Fatal(Win32Error::new(1)));
                }
            };
            let fallback_hotkey = match conflict {
                Hotkey::ShiftSpace => Hotkey::CtrlSpace,
                Hotkey::CtrlSpace => Hotkey::ShiftSpace,
            };
            let fallback = AppSettings::new(
                fallback_hotkey == Hotkey::ShiftSpace,
                fallback_hotkey == Hotkey::CtrlSpace,
            )
            .map_err(|_| HotkeyLoadError::Fatal(Win32Error::new(1)))?;
            match HotkeyManager::new(WinHotkeyBackend::new(hwnd), fallback) {
                Ok(manager) => {
                    store.save(fallback)?;
                    Ok((
                        manager,
                        format!(
                            "{} 단축키가 이미 사용 중이어서 {}만 활성화했습니다.",
                            hotkey_label(conflict),
                            hotkey_label(fallback_hotkey)
                        ),
                    ))
                }
                Err(_) => Err(HotkeyLoadError::BothConflict),
            }
        }
    }
}

enum HotkeyLoadError {
    Fatal(Win32Error),
    BothConflict,
}

impl From<Win32Error> for HotkeyLoadError {
    fn from(error: Win32Error) -> Self {
        Self::Fatal(error)
    }
}

fn message_loop(
    handles: &UiHandles,
    tray: &mut TrayIcon,
    controller: &mut Controller,
    timer: &mut TimerGuard,
) -> Result<i32, Win32Error> {
    let started = Instant::now();
    let mut message = MSG::default();
    loop {
        // Safety: message is a writable MSG owned by this loop; null HWND and zero filters request
        // the current thread's complete queue without borrowing external memory.
        let result = unsafe { GetMessageW(&mut message, None, 0, 0) };
        if result.0 == 0 {
            return Ok(0);
        }
        if result.0 == -1 {
            return Err(Win32Error::new(1));
        }

        if message.message == WM_HOTKEY {
            let hotkey = hotkey_from_id(message.wParam.0 as i32);
            if should_process_hotkey(controller.registered_hotkeys(), hotkey)
                && controller.on_hotkey(started.elapsed())
            {
                timer.start()?;
            }
            continue;
        }

        if message.message == WM_TIMER && should_process_timer(timer.current_id(), message.wParam.0)
        {
            let now = started.elapsed();
            match controller.poll_toggle(now, keys_are_released()) {
                Ok(ControllerEvent::ToggleTimedOut) => {
                    timer.stop();
                    render_best_effort(
                        handles,
                        controller,
                        "입력 해제를 기다리는 시간이 초과되었습니다.",
                    );
                    notify_best_effort(tray, "입력 해제를 기다리는 시간이 초과되었습니다.");
                }
                Ok(ControllerEvent::ToggleSent) | Ok(ControllerEvent::Idle) => timer.stop(),
                Ok(ControllerEvent::Waiting) => {}
                Err(error) => {
                    timer.stop();
                    render_controller_error(handles, tray, controller, error);
                }
            }
            continue;
        }

        if message.message == WM_APP_TRAY {
            match tray.read_event(message.wParam.0, message.lParam.0, controller.settings())? {
                Some(event) if handle_ui_event(handles, tray, controller, event)? => {
                    return Ok(0);
                }
                Some(_) | None => {}
            }
            continue;
        }

        if message.message == WM_DESTROY {
            // Safety: this posts a scalar quit message to the current thread and retains no
            // pointer to Rust-owned data.
            unsafe { PostQuitMessage(0) };
            return Ok(0);
        }

        // read_ui_event must run before IsDialogMessageW: this keeps Escape on the typed Hide
        // path, while IsDialogMessageW still provides normal WS_TABSTOP keyboard navigation.
        if let Some(event) = read_ui_event(&message) {
            if handle_ui_event(handles, tray, controller, event)? {
                return Ok(0);
            }
            continue;
        }

        // Safety: handles.hwnd is the live settings window and message is a valid queue item.
        unsafe {
            if IsDialogMessageW(handles.hwnd, &message).as_bool() {
                continue;
            }
        }
        // Safety: message is initialized by GetMessageW and remains valid for both synchronous
        // translation and dispatch calls; no Rust pointers are embedded in it.
        unsafe {
            let _ = TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }
}

fn handle_ui_event(
    handles: &UiHandles,
    tray: &mut TrayIcon,
    controller: &mut Controller,
    event: UiEvent,
) -> Result<bool, Win32Error> {
    match event {
        UiEvent::SetHotkey(hotkey, enabled) => match controller.set_hotkey(hotkey, enabled) {
            Ok(_) => render_state(
                handles,
                controller.settings(),
                controller.startup_enabled(),
                "설정을 저장했습니다.",
            )?,
            Err(error) => render_controller_error(handles, tray, controller, error),
        },
        UiEvent::SetStartup(enabled) => match controller.set_startup(enabled) {
            Ok(_) => render_state(
                handles,
                controller.settings(),
                controller.startup_enabled(),
                "자동 실행 설정을 저장했습니다.",
            )?,
            Err(error) => render_controller_error(handles, tray, controller, error),
        },
        UiEvent::Hide => hide_window(handles.hwnd),
        UiEvent::Show => show_window(handles.hwnd),
        UiEvent::Exit => return Ok(true),
    }
    Ok(false)
}

fn render_controller_error(
    handles: &UiHandles,
    tray: &mut TrayIcon,
    controller: &Controller,
    error: ControllerError,
) {
    let message = format!("{}", error);
    render_best_effort(handles, controller, &message);
    notify_best_effort(tray, &message);
}

fn render_best_effort(handles: &UiHandles, controller: &Controller, status: &str) {
    let _ = render_state(
        handles,
        controller.settings(),
        controller.startup_enabled(),
        status,
    );
}

fn notify_best_effort(tray: &mut TrayIcon, body: &str) {
    // Notification failure is deliberately non-fatal: the status control remains the source of
    // truth and a shell toast may be unavailable when Explorer is restarting or notifications
    // are disabled.
    let _ = tray.notify("한/영 전환 도우미", body);
}

fn hotkey_from_id(id: i32) -> Option<Hotkey> {
    [Hotkey::ShiftSpace, Hotkey::CtrlSpace]
        .into_iter()
        .find(|&hotkey| hotkey_spec(hotkey).id == id)
}

fn should_process_hotkey(settings: AppSettings, hotkey: Option<Hotkey>) -> bool {
    hotkey.is_some_and(|hotkey| settings.is_enabled(hotkey))
}

fn hotkey_label(hotkey: Hotkey) -> &'static str {
    match hotkey {
        Hotkey::ShiftSpace => "Shift + Space",
        Hotkey::CtrlSpace => "Ctrl + Space",
    }
}

fn show_window(hwnd: HWND) {
    // Safety: hwnd is an app-owned top-level window and these calls pass only scalar handles.
    unsafe {
        let _ = ShowWindow(hwnd, SW_SHOW);
        let _ = SetForegroundWindow(hwnd);
    }
}

fn hide_window(hwnd: HWND) {
    // Safety: hwnd is an app-owned top-level window and ShowWindow does not retain Rust data.
    unsafe {
        let _ = ShowWindow(hwnd, SW_HIDE);
    }
}

fn destroy_window(hwnd: HWND) {
    // Safety: hwnd is the app-owned top-level window; destruction is requested after all child
    // objects and the tray icon have been released by the caller.
    unsafe {
        let _ = windows::Win32::UI::WindowsAndMessaging::DestroyWindow(hwnd);
    }
}

/// Show a native Korean error dialog for startup failures before the message loop exists.
pub fn show_error_message(title: &str, body: &str) {
    let title = wide(title);
    let body = wide(body);
    // Safety: both UTF-16 buffers are NUL-terminated and remain alive for this synchronous call;
    // MessageBoxW copies their text and retains no Rust pointer.
    unsafe {
        let _ = MessageBoxW(
            None,
            PCWSTR(body.as_ptr()),
            PCWSTR(title.as_ptr()),
            MB_OK | MB_ICONERROR,
        );
    }
}

fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}

#[cfg(test)]
mod tests {
    use crate::config::{AppSettings, Hotkey};

    use super::should_process_hotkey;

    #[test]
    fn stale_or_disabled_hotkey_messages_are_ignored() {
        let only_control = AppSettings::new(false, true).expect("one hotkey remains");
        assert!(!should_process_hotkey(
            only_control,
            Some(Hotkey::ShiftSpace)
        ));
        assert!(should_process_hotkey(only_control, Some(Hotkey::CtrlSpace)));
        assert!(!should_process_hotkey(only_control, None));
    }
}
