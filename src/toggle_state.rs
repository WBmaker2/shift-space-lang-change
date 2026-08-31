use std::time::Duration;

pub const TOGGLE_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PollResult {
    Idle,
    Waiting,
    SendToggle,
    TimedOut,
}

#[derive(Debug, Default)]
pub struct ToggleState {
    requested_at: Option<Duration>,
}

impl ToggleState {
    pub fn request(&mut self, now: Duration) -> bool {
        if self.requested_at.is_some() {
            return false;
        }

        self.requested_at = Some(now);
        true
    }

    pub fn poll(&mut self, now: Duration, keys_released: bool) -> PollResult {
        let Some(requested_at) = self.requested_at else {
            return PollResult::Idle;
        };

        if now.saturating_sub(requested_at) >= TOGGLE_TIMEOUT {
            self.requested_at = None;
            return PollResult::TimedOut;
        }

        if keys_released {
            self.requested_at = None;
            return PollResult::SendToggle;
        }

        PollResult::Waiting
    }

    pub fn is_pending(&self) -> bool {
        self.requested_at.is_some()
    }
}
