#![cfg(windows)]

use shift_space_lang_change::platform::windows::single_instance::{
    AcquireResult, SingleInstanceGuard,
};

#[test]
fn a_named_mutex_allows_only_one_primary_guard() {
    let name = format!(r"Local\ShiftSpaceLangChange.Test.{}", std::process::id());

    let first = SingleInstanceGuard::acquire_named(&name).expect("first mutex acquisition");
    let guard = match first {
        AcquireResult::Primary(guard) => guard,
        AcquireResult::AlreadyRunning => panic!("test mutex should not already exist"),
    };

    assert!(matches!(
        SingleInstanceGuard::acquire_named(&name).expect("second mutex acquisition"),
        AcquireResult::AlreadyRunning
    ));

    drop(guard);
}

#[test]
fn invalid_mutex_names_are_rejected_before_win32() {
    assert!(SingleInstanceGuard::acquire_named("").is_err());
    assert!(SingleInstanceGuard::acquire_named("Local\\ShiftSpaceLangChange\0suffix").is_err());
}
