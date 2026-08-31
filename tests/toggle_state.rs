use std::time::Duration;

use shift_space_lang_change::toggle_state::{PollResult, ToggleState};

#[test]
fn one_request_sends_once_after_keys_are_released() {
    let mut state = ToggleState::default();
    assert!(state.request(Duration::ZERO));
    assert!(!state.request(Duration::from_millis(10)));
    assert_eq!(
        state.poll(Duration::from_millis(20), false),
        PollResult::Waiting
    );
    assert_eq!(
        state.poll(Duration::from_millis(30), true),
        PollResult::SendToggle
    );
    assert_eq!(
        state.poll(Duration::from_millis(40), true),
        PollResult::Idle
    );
}

#[test]
fn request_times_out_after_five_seconds() {
    let mut state = ToggleState::default();
    state.request(Duration::ZERO);
    assert_eq!(
        state.poll(Duration::from_secs(5), false),
        PollResult::TimedOut
    );
    assert_eq!(state.poll(Duration::from_secs(6), true), PollResult::Idle);
}
