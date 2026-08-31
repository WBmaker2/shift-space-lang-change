use shift_space_lang_change::config::{AppSettings, Hotkey, SettingsError};

#[test]
fn defaults_enable_both_hotkeys() {
    let settings = AppSettings::default();
    assert_eq!(
        settings.enabled_hotkeys(),
        vec![Hotkey::ShiftSpace, Hotkey::CtrlSpace]
    );
}

#[test]
fn either_single_hotkey_is_valid() {
    assert!(AppSettings::new(true, false).is_ok());
    assert!(AppSettings::new(false, true).is_ok());
}

#[test]
fn disabling_the_last_hotkey_is_rejected() {
    let settings = AppSettings::new(true, false).unwrap();
    assert_eq!(
        settings.with_hotkey(Hotkey::ShiftSpace, false),
        Err(SettingsError::NoHotkeyEnabled)
    );
}

#[test]
fn getters_expose_state_without_mutation() {
    let settings = AppSettings::new(true, false).unwrap();
    assert!(settings.shift_space_enabled());
    assert!(!settings.ctrl_space_enabled());
}
