use shift_space_lang_change::config::Hotkey;
use shift_space_lang_change::ui_model::{
    IDC_CTRL_SPACE, IDC_HIDE, IDC_SHIFT_SPACE, IDC_STARTUP, IDM_EXIT, IDM_SHOW, UiEvent,
    map_command, map_tray_command, tray_event_code,
};

#[test]
fn checkbox_commands_map_to_typed_events() {
    assert_eq!(
        map_command(IDC_SHIFT_SPACE, true),
        Some(UiEvent::SetHotkey(Hotkey::ShiftSpace, true))
    );
    assert_eq!(
        map_command(IDC_CTRL_SPACE, false),
        Some(UiEvent::SetHotkey(Hotkey::CtrlSpace, false))
    );
    assert_eq!(
        map_command(IDC_STARTUP, true),
        Some(UiEvent::SetStartup(true))
    );
    assert_eq!(map_command(IDC_HIDE, false), Some(UiEvent::Hide));
}

#[test]
fn tray_commands_map_to_show_and_exit() {
    assert_eq!(map_tray_command(IDM_SHOW), Some(UiEvent::Show));
    assert_eq!(map_tray_command(IDM_EXIT), Some(UiEvent::Exit));
}

#[test]
fn unknown_commands_are_ignored() {
    assert_eq!(map_command(9999, true), None);
    assert_eq!(map_tray_command(9999), None);
}

#[test]
fn tray_event_code_uses_low_word_for_notify_icon_version_four() {
    let lparam = (0x0042_u32 << 16 | 0x0203_u32) as isize;
    assert_eq!(tray_event_code(lparam), 0x0203);
}
