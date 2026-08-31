use std::collections::BTreeSet;

use shift_space_lang_change::config::{AppSettings, Hotkey};
use shift_space_lang_change::hotkeys::{HotkeyBackend, HotkeyManager};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FakeError;

impl std::fmt::Display for FakeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("fake backend error")
    }
}

impl std::error::Error for FakeError {}

#[derive(Debug, Default)]
struct FakeBackend {
    registered: BTreeSet<Hotkey>,
    fail_register: Option<Hotkey>,
    fail_unregister: Option<Hotkey>,
    calls: Vec<(&'static str, Hotkey)>,
}

impl FakeBackend {
    fn with_failure(hotkey: Hotkey) -> Self {
        Self {
            fail_register: Some(hotkey),
            ..Self::default()
        }
    }

    fn registered(&self) -> BTreeSet<Hotkey> {
        self.registered.clone()
    }

    fn fail_after_unregistering(mut self, hotkey: Hotkey) -> Self {
        self.fail_unregister = Some(hotkey);
        self
    }
}

impl HotkeyBackend for FakeBackend {
    type Error = FakeError;

    fn register(&mut self, hotkey: Hotkey) -> Result<(), Self::Error> {
        self.calls.push(("register", hotkey));
        if self.fail_register == Some(hotkey) {
            return Err(FakeError);
        }
        self.registered.insert(hotkey);
        Ok(())
    }

    fn unregister(&mut self, hotkey: Hotkey) -> Result<(), Self::Error> {
        self.calls.push(("unregister", hotkey));
        if self.fail_unregister == Some(hotkey) {
            return Err(FakeError);
        }
        self.registered.remove(&hotkey);
        Ok(())
    }
}

#[test]
fn failed_registration_keeps_previous_active_configuration() {
    let backend = FakeBackend::with_failure(Hotkey::CtrlSpace);
    let initial = AppSettings::new(true, false).unwrap();
    let mut manager = HotkeyManager::new(backend, initial).unwrap();

    let desired = AppSettings::default();
    let error = manager.apply(desired).unwrap_err();

    assert!(matches!(
        error,
        shift_space_lang_change::hotkeys::ApplyError::Register {
            hotkey: Hotkey::CtrlSpace,
            ..
        }
    ));
    assert_eq!(manager.active_settings(), initial);
    assert_eq!(manager.backend().registered(), [Hotkey::ShiftSpace].into());
    assert_eq!(
        manager.backend().calls,
        vec![
            ("register", Hotkey::ShiftSpace),
            ("register", Hotkey::CtrlSpace)
        ]
    );
}

#[test]
fn successful_change_updates_active_configuration() {
    let backend = FakeBackend::default();
    let mut manager = HotkeyManager::new(backend, AppSettings::default()).unwrap();
    let desired = AppSettings::new(false, true).unwrap();
    manager.apply(desired).unwrap();
    assert_eq!(manager.active_settings(), desired);
}

#[test]
fn failed_middle_registration_rolls_back_all_new_registrations() {
    let backend = FakeBackend::with_failure(Hotkey::CtrlSpace);
    let initial = AppSettings::new(true, false).unwrap();
    let mut manager = HotkeyManager::new(backend, initial).unwrap();
    let desired = AppSettings::default();

    manager.apply(desired).unwrap_err();
    assert_eq!(manager.active_settings(), initial);
    assert_eq!(manager.backend().registered(), [Hotkey::ShiftSpace].into());
    assert_eq!(
        manager.backend().calls,
        vec![
            ("register", Hotkey::ShiftSpace),
            ("register", Hotkey::CtrlSpace)
        ]
    );
}

#[test]
fn unregister_failure_rolls_back_previous_removal_and_keeps_settings() {
    let backend = FakeBackend::default().fail_after_unregistering(Hotkey::CtrlSpace);
    let initial = AppSettings::default();
    let mut manager = HotkeyManager::new(backend, initial).unwrap();
    let desired = AppSettings::new(true, false).unwrap();

    manager.apply(desired).unwrap_err();
    assert_eq!(manager.active_settings(), initial);
    assert_eq!(
        manager.backend().registered(),
        [Hotkey::ShiftSpace, Hotkey::CtrlSpace].into()
    );
    assert_eq!(
        manager.backend().calls,
        vec![
            ("register", Hotkey::ShiftSpace),
            ("register", Hotkey::CtrlSpace),
            ("unregister", Hotkey::CtrlSpace),
        ]
    );
}
