use super::error::Win32Error;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{KillTimer, SetTimer};

const FIRST_TIMER_ID: usize = 0x5103;
const TIMER_INTERVAL_MS: u32 = 15;

/// Generation-aware WM_TIMER ownership for one app window.
pub(super) struct TimerGuard {
    hwnd: HWND,
    active: bool,
    current_id: Option<usize>,
    next_id: usize,
}

impl TimerGuard {
    pub(super) fn new(hwnd: HWND) -> Self {
        Self {
            hwnd,
            active: false,
            current_id: None,
            next_id: FIRST_TIMER_ID,
        }
    }

    pub(super) fn start(&mut self) -> Result<(), Win32Error> {
        if self.active {
            return Ok(());
        }
        // Each pending cycle gets a new ID, so queued WM_TIMER messages from an older cycle
        // cannot match the new active generation.
        let id = self.next_timer_id();
        // Safety: hwnd is the app-owned settings window and the null callback requests WM_TIMER
        // messages on this thread; the scalar timer id and interval are valid Win32 values.
        let timer = unsafe { SetTimer(Some(self.hwnd), id, TIMER_INTERVAL_MS, None) };
        if timer == 0 {
            Err(Win32Error::new(1))
        } else {
            self.active = true;
            self.current_id = Some(id);
            Ok(())
        }
    }

    pub(super) fn stop(&mut self) {
        if !self.active {
            return;
        }
        // A queued WM_TIMER from this generation may remain after KillTimer. Its generation ID
        // cannot match the next active timer and is ignored by should_process_timer.
        // Safety: current_id belongs to this app-owned HWND and was passed to SetTimer.
        unsafe {
            let _ = KillTimer(Some(self.hwnd), self.current_id.unwrap_or(0));
        }
        self.active = false;
        self.current_id = None;
    }

    pub(super) fn current_id(&self) -> Option<usize> {
        self.current_id.filter(|_| self.active)
    }

    fn next_timer_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id = next_generation_id(id);
        id
    }
}

impl Drop for TimerGuard {
    fn drop(&mut self) {
        self.stop();
    }
}

fn next_generation_id(current: usize) -> usize {
    let mut candidate = current.wrapping_add(1);
    loop {
        if candidate != 0 && candidate != 0x5101 && candidate != 0x5102 {
            return candidate;
        }
        candidate = candidate.wrapping_add(1);
    }
}

pub(super) fn should_process_timer(current_id: Option<usize>, id: usize) -> bool {
    current_id == Some(id)
}

#[cfg(test)]
mod tests {
    use super::{FIRST_TIMER_ID, next_generation_id, should_process_timer};

    #[test]
    fn stale_or_inactive_timer_messages_are_ignored() {
        assert!(!should_process_timer(None, FIRST_TIMER_ID));
        assert!(!should_process_timer(
            Some(FIRST_TIMER_ID),
            FIRST_TIMER_ID + 1
        ));
        assert!(!should_process_timer(
            Some(FIRST_TIMER_ID + 1),
            FIRST_TIMER_ID
        ));
        assert!(should_process_timer(Some(FIRST_TIMER_ID), FIRST_TIMER_ID));
    }

    #[test]
    fn timer_generations_advance_and_skip_reserved_ids() {
        assert_eq!(next_generation_id(FIRST_TIMER_ID), FIRST_TIMER_ID + 1);
        assert_ne!(next_generation_id(0x5100), 0x5101);
        assert_ne!(next_generation_id(0x5101), 0x5102);
        assert_ne!(next_generation_id(usize::MAX), 0);
    }
}
