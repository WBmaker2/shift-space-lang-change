use crate::config::Hotkey;

/// Events emitted by the native settings window and its tray menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiEvent {
    SetHotkey(Hotkey, bool),
    SetStartup(bool),
    Hide,
    Show,
    Exit,
}

pub const IDC_SHIFT_SPACE: i32 = 1001;
pub const IDC_CTRL_SPACE: i32 = 1002;
pub const IDC_STARTUP: i32 = 1003;
pub const IDC_HIDE: i32 = 1004;
pub const IDC_STATUS: i32 = 1005;

pub const IDM_SHOW: usize = 2001;
pub const IDM_EXIT: usize = 2002;

pub fn map_command(id: i32, checked: bool) -> Option<UiEvent> {
    match id {
        IDC_SHIFT_SPACE => Some(UiEvent::SetHotkey(Hotkey::ShiftSpace, checked)),
        IDC_CTRL_SPACE => Some(UiEvent::SetHotkey(Hotkey::CtrlSpace, checked)),
        IDC_STARTUP => Some(UiEvent::SetStartup(checked)),
        IDC_HIDE => Some(UiEvent::Hide),
        _ => None,
    }
}

/// Convert a button's `WM_COMMAND` payload into a typed event.
///
/// A parent window receives button notifications synchronously, so callers may post the raw
/// `WPARAM` to their own queue and use this function when the queued message is handled. The
/// checkbox state is supplied separately because it belongs to the native control.
pub fn map_command_notification(wparam: usize, checked: bool) -> Option<UiEvent> {
    let notification = ((wparam >> 16) & 0xffff) as u32;
    if notification != 0 {
        return None;
    }
    map_command((wparam & 0xffff) as i32, checked)
}

/// Convert the compact payload posted by the settings window procedure.
///
/// Bit 16 records the checkbox state captured while the synchronous `WM_COMMAND` was received;
/// this avoids reading a later state if a user clicks a checkbox repeatedly before the queue is
/// drained.
pub fn map_queued_command(wparam: usize) -> Option<UiEvent> {
    let checked = (wparam & (1 << 16)) != 0;
    map_command((wparam & 0xffff) as i32, checked)
}

pub fn map_tray_command(id: usize) -> Option<UiEvent> {
    match id {
        IDM_SHOW => Some(UiEvent::Show),
        IDM_EXIT => Some(UiEvent::Exit),
        _ => None,
    }
}

/// Return the notification mouse event from the LOWORD of a version-4 tray callback lParam.
pub const fn tray_event_code(lparam: isize) -> u32 {
    (lparam as u32) & 0xffff
}
