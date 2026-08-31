use std::cell::RefCell;
use std::collections::{BTreeSet, VecDeque};
use std::rc::Rc;

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
    unregister_error_after_side_effect: bool,
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

    fn with_initial_rollback_failure() -> Self {
        Self {
            fail_register: Some(Hotkey::CtrlSpace),
            fail_unregister: Some(Hotkey::ShiftSpace),
            ..Self::default()
        }
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
        if self.fail_unregister == Some(hotkey) && !self.unregister_error_after_side_effect {
            return Err(FakeError);
        }
        self.registered.remove(&hotkey);
        if self.fail_unregister == Some(hotkey) {
            return Err(FakeError);
        }
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
            ("register", Hotkey::CtrlSpace),
        ]
    );
}

#[test]
fn rollback_restores_removed_hotkeys_before_unregistration_of_added_hotkeys() {
    let initial = AppSettings::new(true, false).unwrap();
    let backend = FakeBackend {
        fail_unregister: Some(Hotkey::ShiftSpace),
        ..FakeBackend::default()
    };
    let mut manager = HotkeyManager::new(backend, initial).unwrap();
    let desired = AppSettings::new(false, true).unwrap();

    manager.apply(desired).unwrap_err();
    assert_eq!(manager.active_settings(), initial);
    assert_eq!(manager.backend().registered(), [Hotkey::ShiftSpace].into());
    assert_eq!(
        manager.backend().calls,
        vec![
            ("register", Hotkey::ShiftSpace),
            ("register", Hotkey::CtrlSpace),
            ("unregister", Hotkey::ShiftSpace),
            ("register", Hotkey::ShiftSpace),
            ("unregister", Hotkey::CtrlSpace),
        ]
    );
}

#[test]
fn unregister_error_after_side_effect_restores_the_failed_hotkey() {
    let backend = FakeBackend {
        fail_unregister: Some(Hotkey::CtrlSpace),
        unregister_error_after_side_effect: true,
        ..FakeBackend::default()
    };
    let initial = AppSettings::default();
    let mut manager = HotkeyManager::new(backend, initial).unwrap();

    manager
        .apply(AppSettings::new(true, false).unwrap())
        .unwrap_err();
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
            ("register", Hotkey::CtrlSpace),
        ]
    );
}

#[test]
fn initial_registration_rollback_failure_is_reported() {
    let result = HotkeyManager::new(
        FakeBackend::with_initial_rollback_failure(),
        AppSettings::default(),
    );
    let error = match result {
        Ok(_) => panic!("initial registration should fail"),
        Err(error) => error,
    };

    assert!(matches!(
        error,
        shift_space_lang_change::hotkeys::ApplyError::Rollback {
            operation: "unregister",
            hotkey: Hotkey::ShiftSpace,
            ..
        }
    ));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Register(Hotkey),
    Unregister(Hotkey),
}

#[derive(Debug, Default)]
struct ScriptedTrace {
    registered: BTreeSet<Hotkey>,
    calls: Vec<Operation>,
}

#[derive(Debug)]
struct ScriptedBackend {
    trace: Rc<RefCell<ScriptedTrace>>,
    outcomes: VecDeque<Result<(), FakeError>>,
}

impl ScriptedBackend {
    fn new(
        outcomes: impl IntoIterator<Item = Result<(), FakeError>>,
    ) -> (Self, Rc<RefCell<ScriptedTrace>>) {
        let trace = Rc::new(RefCell::new(ScriptedTrace::default()));
        (
            Self {
                trace: Rc::clone(&trace),
                outcomes: outcomes.into_iter().collect(),
            },
            trace,
        )
    }

    fn outcome(&mut self) -> Result<(), FakeError> {
        self.outcomes.pop_front().unwrap_or(Ok(()))
    }
}

impl HotkeyBackend for ScriptedBackend {
    type Error = FakeError;

    fn register(&mut self, hotkey: Hotkey) -> Result<(), Self::Error> {
        self.trace
            .borrow_mut()
            .calls
            .push(Operation::Register(hotkey));
        let outcome = self.outcome();
        if outcome.is_ok() {
            self.trace.borrow_mut().registered.insert(hotkey);
        }
        outcome
    }

    fn unregister(&mut self, hotkey: Hotkey) -> Result<(), Self::Error> {
        self.trace
            .borrow_mut()
            .calls
            .push(Operation::Unregister(hotkey));
        let outcome = self.outcome();
        if outcome.is_ok() {
            self.trace.borrow_mut().registered.remove(&hotkey);
        }
        outcome
    }
}

#[test]
fn rollback_continues_after_first_restore_error_and_cleans_added_hotkey() {
    let (backend, trace) = ScriptedBackend::new([
        Ok(()),         // initial Shift + Space registration
        Ok(()),         // add Ctrl + Space
        Err(FakeError), // remove Shift + Space fails
        Err(FakeError), // restoring Shift + Space is the first rollback error
        Ok(()),         // removing added Ctrl + Space still runs
    ]);
    let initial = AppSettings::new(true, false).unwrap();
    let mut manager = HotkeyManager::new(backend, initial).unwrap();

    let error = manager
        .apply(AppSettings::new(false, true).unwrap())
        .unwrap_err();

    assert!(matches!(
        error,
        shift_space_lang_change::hotkeys::ApplyError::Rollback {
            operation: "register",
            hotkey: Hotkey::ShiftSpace,
            ..
        }
    ));
    assert_eq!(manager.active_settings(), initial);
    assert_eq!(manager.registered_hotkeys(), [Hotkey::ShiftSpace].into());
    assert_eq!(trace.borrow().registered, [Hotkey::ShiftSpace].into());
    assert_eq!(
        trace.borrow().calls,
        vec![
            Operation::Register(Hotkey::ShiftSpace),
            Operation::Register(Hotkey::CtrlSpace),
            Operation::Unregister(Hotkey::ShiftSpace),
            Operation::Register(Hotkey::ShiftSpace),
            Operation::Unregister(Hotkey::CtrlSpace),
        ]
    );
}

#[test]
fn drop_retries_added_registration_left_by_rollback_failure() {
    let (backend, trace) = ScriptedBackend::new([
        Ok(()),         // initial Shift + Space registration
        Ok(()),         // add Ctrl + Space
        Err(FakeError), // remove Shift + Space fails
        Ok(()),         // restoring Shift + Space
        Err(FakeError), // rollback cannot remove added Ctrl + Space
        Ok(()),         // Drop retries Ctrl + Space
        Ok(()),         // Drop releases Shift + Space
    ]);
    let initial = AppSettings::new(true, false).unwrap();
    let mut manager = HotkeyManager::new(backend, initial).unwrap();

    let error = manager
        .apply(AppSettings::new(false, true).unwrap())
        .unwrap_err();
    assert!(matches!(
        error,
        shift_space_lang_change::hotkeys::ApplyError::Rollback {
            operation: "unregister",
            hotkey: Hotkey::CtrlSpace,
            ..
        }
    ));
    assert_eq!(manager.active_settings(), initial);
    assert_eq!(
        manager.registered_hotkeys(),
        [Hotkey::ShiftSpace, Hotkey::CtrlSpace].into()
    );

    drop(manager);
    assert_eq!(trace.borrow().registered, BTreeSet::new());
    assert_eq!(
        trace.borrow().calls,
        vec![
            Operation::Register(Hotkey::ShiftSpace),
            Operation::Register(Hotkey::CtrlSpace),
            Operation::Unregister(Hotkey::ShiftSpace),
            Operation::Register(Hotkey::ShiftSpace),
            Operation::Unregister(Hotkey::CtrlSpace),
            Operation::Unregister(Hotkey::CtrlSpace),
            Operation::Unregister(Hotkey::ShiftSpace),
        ]
    );
}
