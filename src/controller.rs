use std::fmt;
use std::time::Duration;

use crate::config::{AppSettings, Hotkey, SettingsError};
use crate::hotkeys::{ApplyError, HotkeyBackend, HotkeyManager};
use crate::ports::{ImeSender, SettingsStore, StartupController};
use crate::toggle_state::{PollResult, ToggleState};

/// Platform-independent events emitted by the controller's hotkey polling loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControllerEvent {
    Idle,
    Waiting,
    ToggleSent,
    ToggleTimedOut,
}

/// Errors are deliberately represented as user-facing operation categories. Platform adapters
/// may expose their own detailed error text through Display without coupling the controller to
/// Win32 types.
#[derive(Debug, PartialEq, Eq)]
pub enum ControllerError {
    InvalidSettings(SettingsError),
    HotkeyApply(String),
    SettingsSave(String),
    SettingsRollback { save: String, rollback: String },
    Startup(String),
    Ime(String),
}

impl fmt::Display for ControllerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSettings(error) => {
                write!(formatter, "설정이 올바르지 않습니다: {error:?}")
            }
            Self::HotkeyApply(error) => write!(formatter, "단축키 적용 실패: {error}"),
            Self::SettingsSave(error) => write!(formatter, "설정 저장 실패: {error}"),
            Self::SettingsRollback { save, rollback } => write!(
                formatter,
                "설정 저장 실패({save}), 이전 단축키 복원도 실패했습니다({rollback})"
            ),
            Self::Startup(error) => write!(formatter, "자동 실행 설정 실패: {error}"),
            Self::Ime(error) => write!(formatter, "한/영 전환 입력 실패: {error}"),
        }
    }
}

impl std::error::Error for ControllerError {}

/// Coordinates persistent settings, registered hotkeys, startup state, and IME toggles.
pub struct AppController<B, S, A, I>
where
    B: HotkeyBackend,
    S: SettingsStore,
    A: StartupController,
    I: ImeSender,
{
    hotkeys: HotkeyManager<B>,
    store: S,
    startup: A,
    ime: I,
    settings: AppSettings,
    startup_enabled: bool,
    toggle: ToggleState,
}

impl<B, S, A, I> AppController<B, S, A, I>
where
    B: HotkeyBackend,
    S: SettingsStore,
    A: StartupController,
    I: ImeSender,
{
    pub fn new(
        hotkeys: HotkeyManager<B>,
        store: S,
        startup: A,
        ime: I,
        startup_enabled: bool,
    ) -> Self {
        let settings = hotkeys.active_settings();
        Self {
            hotkeys,
            store,
            startup,
            ime,
            settings,
            startup_enabled,
            toggle: ToggleState::default(),
        }
    }

    /// Apply a checkbox edit immediately, persisting it only after registration succeeds.
    pub fn set_hotkey(
        &mut self,
        hotkey: Hotkey,
        enabled: bool,
    ) -> Result<AppSettings, ControllerError> {
        let previous = self.settings;
        let desired = previous
            .with_hotkey(hotkey, enabled)
            .map_err(ControllerError::InvalidSettings)?;

        self.hotkeys
            .apply(desired)
            .map_err(|error| ControllerError::HotkeyApply(apply_error_text(error)))?;

        if let Err(error) = self.store.save(desired) {
            let save = error.to_string();
            if let Err(rollback_error) = self.hotkeys.apply(previous) {
                self.settings = self.hotkeys.active_settings();
                return Err(ControllerError::SettingsRollback {
                    save,
                    rollback: apply_error_text(rollback_error),
                });
            }
            self.settings = previous;
            return Err(ControllerError::SettingsSave(save));
        }

        self.settings = desired;
        Ok(desired)
    }

    /// Change startup registration and retain the actual registry state if the write fails.
    pub fn set_startup(&mut self, enabled: bool) -> Result<bool, ControllerError> {
        let before = self
            .startup
            .is_enabled()
            .map_err(|error| ControllerError::Startup(error.to_string()))?;
        self.startup_enabled = before;

        if let Err(error) = self.startup.set_enabled(enabled) {
            let write_error = error.to_string();
            match self.startup.is_enabled() {
                Ok(actual) => self.startup_enabled = actual,
                Err(reread_error) => {
                    return Err(ControllerError::Startup(format!(
                        "{write_error}; 실제 상태 확인도 실패했습니다: {reread_error}"
                    )));
                }
            }
            return Err(ControllerError::Startup(write_error));
        }

        self.startup_enabled = enabled;
        Ok(enabled)
    }

    /// Begin waiting for the triggering modifier and space keys to be released.
    pub fn on_hotkey(&mut self, now: Duration) -> bool {
        self.toggle.request(now)
    }

    /// Advance the release state machine, sending exactly one IME input per completed request.
    pub fn poll_toggle(
        &mut self,
        now: Duration,
        keys_released: bool,
    ) -> Result<ControllerEvent, ControllerError> {
        match self.toggle.poll(now, keys_released) {
            PollResult::Idle => Ok(ControllerEvent::Idle),
            PollResult::Waiting => Ok(ControllerEvent::Waiting),
            PollResult::TimedOut => Ok(ControllerEvent::ToggleTimedOut),
            PollResult::SendToggle => {
                self.ime
                    .send_toggle()
                    .map_err(|error| ControllerError::Ime(error.to_string()))?;
                Ok(ControllerEvent::ToggleSent)
            }
        }
    }

    pub fn settings(&self) -> AppSettings {
        self.settings
    }

    pub fn startup_enabled(&self) -> bool {
        self.startup_enabled
    }

    /// Return the registration state used by the manager for UI reconciliation and diagnostics.
    pub fn registered_hotkeys(&self) -> AppSettings {
        self.hotkeys.active_settings()
    }
}

fn apply_error_text<E: fmt::Display>(error: ApplyError<E>) -> String {
    match error {
        ApplyError::Register { hotkey, source } => {
            format!("{hotkey:?} 등록: {source}")
        }
        ApplyError::Unregister { hotkey, source } => {
            format!("{hotkey:?} 해제: {source}")
        }
        ApplyError::Rollback {
            operation,
            hotkey,
            source,
        } => format!("복원 중 {operation} {hotkey:?}: {source}"),
    }
}
