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

pub fn map_tray_command(id: usize) -> Option<UiEvent> {
    match id {
        IDM_SHOW => Some(UiEvent::Show),
        IDM_EXIT => Some(UiEvent::Exit),
        _ => None,
    }
}
