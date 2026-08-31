use std::collections::BTreeSet;
use std::time::Duration;

use shift_space_lang_change::config::{AppSettings, Hotkey};
use shift_space_lang_change::controller::{AppController, ControllerEvent};
use shift_space_lang_change::hotkeys::{HotkeyBackend, HotkeyManager};
use shift_space_lang_change::ports::{ImeSender, SettingsStore, StartupController};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FakeError;

impl std::fmt::Display for FakeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("fake error")
    }
}

impl std::error::Error for FakeError {}

#[derive(Debug, Default)]
struct FakeBackend {
    registered: BTreeSet<Hotkey>,
}

impl HotkeyBackend for FakeBackend {
    type Error = FakeError;

    fn register(&mut self, hotkey: Hotkey) -> Result<(), Self::Error> {
        self.registered.insert(hotkey);
        Ok(())
    }

    fn unregister(&mut self, hotkey: Hotkey) -> Result<(), Self::Error> {
        self.registered.remove(&hotkey);
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct FakeStore {
    saved: AppSettings,
    fail_save: bool,
}

impl SettingsStore for FakeStore {
    type Error = FakeError;

    fn load(&self) -> Result<AppSettings, Self::Error> {
        Ok(self.saved)
    }

    fn save(&self, settings: AppSettings) -> Result<(), Self::Error> {
        if self.fail_save {
            Err(FakeError)
        } else {
            let _ = settings;
            Ok(())
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct FakeStartup {
    actual: bool,
    fail: bool,
}

impl StartupController for FakeStartup {
    type Error = FakeError;

    fn is_enabled(&self) -> Result<bool, Self::Error> {
        Ok(self.actual)
    }

    fn set_enabled(&self, enabled: bool) -> Result<(), Self::Error> {
        if self.fail {
            Err(FakeError)
        } else {
            let _ = enabled;
            Ok(())
        }
    }
}

#[derive(Debug, Default)]
struct FakeIme {
    sends: usize,
}

impl ImeSender for FakeIme {
    type Error = FakeError;

    fn send_toggle(&mut self) -> Result<(), Self::Error> {
        self.sends += 1;
        Ok(())
    }
}

fn fixture(settings: AppSettings) -> AppController<FakeBackend, FakeStore, FakeStartup, FakeIme> {
    let manager = HotkeyManager::new(FakeBackend::default(), settings).unwrap();
    AppController::new(
        manager,
        FakeStore {
            saved: settings,
            fail_save: false,
        },
        FakeStartup {
            actual: false,
            fail: false,
        },
        FakeIme::default(),
        false,
    )
}

fn fixture_with_save_failure(
    settings: AppSettings,
) -> AppController<FakeBackend, FakeStore, FakeStartup, FakeIme> {
    let manager = HotkeyManager::new(FakeBackend::default(), settings).unwrap();
    AppController::new(
        manager,
        FakeStore {
            saved: settings,
            fail_save: true,
        },
        FakeStartup {
            actual: false,
            fail: false,
        },
        FakeIme::default(),
        false,
    )
}

fn fixture_with_startup_failure(
    settings: AppSettings,
) -> AppController<FakeBackend, FakeStore, FakeStartup, FakeIme> {
    let manager = HotkeyManager::new(FakeBackend::default(), settings).unwrap();
    AppController::new(
        manager,
        FakeStore {
            saved: settings,
            fail_save: false,
        },
        FakeStartup {
            actual: false,
            fail: true,
        },
        FakeIme::default(),
        false,
    )
}

#[test]
fn successful_hotkey_change_is_registered_then_saved() {
    let mut controller = fixture(AppSettings::default());
    let actual = controller.set_hotkey(Hotkey::ShiftSpace, false).unwrap();
    assert_eq!(actual, AppSettings::new(false, true).unwrap());
    assert_eq!(controller.settings(), actual);
}

#[test]
fn save_failure_restores_previous_hotkeys_and_ui_state() {
    let mut controller = fixture_with_save_failure(AppSettings::default());
    assert!(controller.set_hotkey(Hotkey::ShiftSpace, false).is_err());
    assert_eq!(controller.settings(), AppSettings::default());
    assert_eq!(
        controller.registered_hotkeys(),
        [Hotkey::ShiftSpace, Hotkey::CtrlSpace].into()
    );
}

#[test]
fn released_keys_send_exactly_one_ime_toggle() {
    let mut controller = fixture(AppSettings::default());
    controller.on_hotkey(Duration::ZERO);
    assert_eq!(
        controller
            .poll_toggle(Duration::from_millis(20), false)
            .unwrap(),
        ControllerEvent::Waiting
    );
    assert_eq!(
        controller
            .poll_toggle(Duration::from_millis(30), true)
            .unwrap(),
        ControllerEvent::ToggleSent
    );
}

#[test]
fn startup_failure_rereads_actual_state() {
    let mut controller = fixture_with_startup_failure(AppSettings::default());
    assert!(controller.set_startup(true).is_err());
    assert!(!controller.startup_enabled());
}

#[test]
fn pending_toggle_times_out_after_five_seconds() {
    let mut controller = fixture(AppSettings::default());
    controller.on_hotkey(Duration::ZERO);
    assert_eq!(
        controller
            .poll_toggle(Duration::from_secs(5), false)
            .unwrap(),
        ControllerEvent::ToggleTimedOut
    );
}
