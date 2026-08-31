use std::collections::BTreeSet;

use crate::config::{AppSettings, Hotkey};

/// Win32 errors that can occur while registering the product hotkeys.
pub const ERROR_HOTKEY_ALREADY_REGISTERED: u32 = 1409;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotkeyErrorClass {
    /// The requested combination is already owned by another application.
    AlreadyRegistered,
    /// The operation failed for a reason that must abort startup or the update.
    Fatal,
}

/// Classify a raw Win32 error without depending on Windows APIs.
pub const fn classify_hotkey_error(code: u32) -> HotkeyErrorClass {
    if code == ERROR_HOTKEY_ALREADY_REGISTERED {
        HotkeyErrorClass::AlreadyRegistered
    } else {
        HotkeyErrorClass::Fatal
    }
}

/// Exposes only the provenance needed by the platform-neutral startup policy.
pub trait HotkeyErrorSource {
    fn code(&self) -> u32;
    fn is_win32(&self) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartupHotkeyPhase {
    Initial,
    Fallback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartupHotkeyDecision {
    Fallback { conflicting: Hotkey },
    BothConflict,
    Fatal,
}

/// Apply the same startup policy to initial and fallback registration failures.
pub fn classify_startup_error<E: HotkeyErrorSource>(
    phase: StartupHotkeyPhase,
    error: &ApplyError<E>,
) -> StartupHotkeyDecision {
    let ApplyError::Register { hotkey, source } = error else {
        return StartupHotkeyDecision::Fatal;
    };
    if !source.is_win32()
        || classify_hotkey_error(source.code()) != HotkeyErrorClass::AlreadyRegistered
    {
        return StartupHotkeyDecision::Fatal;
    }
    match phase {
        StartupHotkeyPhase::Initial => StartupHotkeyDecision::Fallback {
            conflicting: *hotkey,
        },
        StartupHotkeyPhase::Fallback => StartupHotkeyDecision::BothConflict,
    }
}

pub trait HotkeyBackend {
    type Error: std::error::Error + Send + Sync + 'static;

    fn register(&mut self, hotkey: Hotkey) -> Result<(), Self::Error>;
    fn unregister(&mut self, hotkey: Hotkey) -> Result<(), Self::Error>;
}

#[derive(Debug)]
pub enum ApplyError<E> {
    Register {
        hotkey: Hotkey,
        source: E,
    },
    Unregister {
        hotkey: Hotkey,
        source: E,
    },
    Rollback {
        operation: &'static str,
        hotkey: Hotkey,
        source: E,
    },
}

pub struct HotkeyManager<B: HotkeyBackend> {
    backend: B,
    active: AppSettings,
    registered: BTreeSet<Hotkey>,
}

impl<B: HotkeyBackend> Drop for HotkeyManager<B> {
    fn drop(&mut self) {
        // Best-effort release keeps a partially initialized process from leaving registrations
        // behind. The Windows backend owns no Rust resources that need a second shutdown pass.
        let registered: Vec<_> = self.registered.iter().copied().rev().collect();
        for hotkey in registered {
            if self.backend.unregister(hotkey).is_ok() {
                self.registered.remove(&hotkey);
            }
        }
    }
}

impl<B: HotkeyBackend> HotkeyManager<B> {
    pub fn new(mut backend: B, active: AppSettings) -> Result<Self, ApplyError<B::Error>> {
        let mut registered = BTreeSet::new();
        for hotkey in active.enabled_hotkeys() {
            if let Err(source) = backend.register(hotkey) {
                if let Some(error) = rollback_registered(&mut backend, &mut registered) {
                    return Err(error);
                }
                return Err(ApplyError::Register { hotkey, source });
            }
            registered.insert(hotkey);
        }
        Ok(Self {
            backend,
            active,
            registered,
        })
    }

    pub fn apply(&mut self, desired: AppSettings) -> Result<(), ApplyError<B::Error>> {
        let target: BTreeSet<_> = desired.enabled_hotkeys().into_iter().collect();
        let before = self.registered.clone();
        let additions: Vec<_> = target.difference(&before).copied().collect();
        let removals: Vec<_> = before.difference(&target).copied().collect();

        for hotkey in additions {
            if let Err(source) = self.backend.register(hotkey) {
                let operation = ApplyError::Register { hotkey, source };
                return Err(self.rollback_or_original(&before, std::iter::empty(), operation));
            }
            self.registered.insert(hotkey);
        }

        for hotkey in removals {
            if let Err(source) = self.backend.unregister(hotkey) {
                let operation = ApplyError::Unregister { hotkey, source };
                return Err(self.rollback_or_original(&before, [hotkey], operation));
            }
            self.registered.remove(&hotkey);
        }

        self.active = desired;
        Ok(())
    }

    pub fn active_settings(&self) -> AppSettings {
        self.active
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }

    /// Return the hotkeys that the backend has successfully registered.
    pub fn registered_hotkeys(&self) -> BTreeSet<Hotkey> {
        self.registered.clone()
    }

    fn rollback_or_original(
        &mut self,
        target: &BTreeSet<Hotkey>,
        force_register: impl IntoIterator<Item = Hotkey>,
        original: ApplyError<B::Error>,
    ) -> ApplyError<B::Error> {
        match self.rollback_to(target, force_register) {
            Ok(()) => original,
            Err(error) => error,
        }
    }

    fn rollback_to(
        &mut self,
        target: &BTreeSet<Hotkey>,
        force_register: impl IntoIterator<Item = Hotkey>,
    ) -> Result<(), ApplyError<B::Error>> {
        let current = self.registered.clone();
        let mut additions: BTreeSet<_> = target.difference(&current).copied().collect();
        additions.extend(force_register);
        let mut removals: Vec<_> = current.difference(target).copied().collect();
        removals.reverse();
        let mut first_error = None;
        for hotkey in additions {
            match self.backend.register(hotkey) {
                Ok(()) => {
                    self.registered.insert(hotkey);
                }
                Err(source) => {
                    if first_error.is_none() {
                        first_error = Some(ApplyError::Rollback {
                            operation: "register",
                            hotkey,
                            source,
                        });
                    }
                }
            }
        }
        for hotkey in removals {
            match self.backend.unregister(hotkey) {
                Ok(()) => {
                    self.registered.remove(&hotkey);
                }
                Err(source) => {
                    if first_error.is_none() {
                        first_error = Some(ApplyError::Rollback {
                            operation: "unregister",
                            hotkey,
                            source,
                        });
                    }
                }
            }
        }
        first_error.map_or(Ok(()), Err)
    }
}

fn rollback_registered<B: HotkeyBackend>(
    backend: &mut B,
    registered: &mut BTreeSet<Hotkey>,
) -> Option<ApplyError<B::Error>> {
    let hotkeys: Vec<_> = registered.iter().copied().rev().collect();
    let mut first_error = None;
    for hotkey in hotkeys {
        match backend.unregister(hotkey) {
            Ok(()) => {
                registered.remove(&hotkey);
            }
            Err(source) => {
                if first_error.is_none() {
                    first_error = Some(ApplyError::Rollback {
                        operation: "unregister",
                        hotkey,
                        source,
                    });
                }
            }
        }
    }
    first_error
}

#[cfg(test)]
mod tests {
    use std::fmt;

    use super::{
        ApplyError, ERROR_HOTKEY_ALREADY_REGISTERED, Hotkey, HotkeyErrorClass, HotkeyErrorSource,
        StartupHotkeyDecision, StartupHotkeyPhase, classify_hotkey_error, classify_startup_error,
    };

    #[derive(Debug, Clone, Copy)]
    struct Source {
        code: u32,
        is_win32: bool,
    }

    impl fmt::Display for Source {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(formatter, "error {}", self.code)
        }
    }

    impl std::error::Error for Source {}

    impl HotkeyErrorSource for Source {
        fn code(&self) -> u32 {
            self.code
        }

        fn is_win32(&self) -> bool {
            self.is_win32
        }
    }

    #[test]
    fn only_already_registered_is_a_conflict() {
        assert_eq!(
            classify_hotkey_error(ERROR_HOTKEY_ALREADY_REGISTERED),
            HotkeyErrorClass::AlreadyRegistered
        );
        assert_eq!(classify_hotkey_error(5), HotkeyErrorClass::Fatal);
        assert_eq!(classify_hotkey_error(6), HotkeyErrorClass::Fatal);
        assert_eq!(classify_hotkey_error(0xdead_beef), HotkeyErrorClass::Fatal);
    }

    #[test]
    fn startup_policy_only_falls_back_for_initial_win32_1409() {
        let conflict = ApplyError::Register {
            hotkey: Hotkey::ShiftSpace,
            source: Source {
                code: ERROR_HOTKEY_ALREADY_REGISTERED,
                is_win32: true,
            },
        };
        assert_eq!(
            classify_startup_error(StartupHotkeyPhase::Initial, &conflict),
            StartupHotkeyDecision::Fallback {
                conflicting: Hotkey::ShiftSpace
            }
        );
        assert_eq!(
            classify_startup_error(StartupHotkeyPhase::Fallback, &conflict),
            StartupHotkeyDecision::BothConflict
        );
    }

    #[test]
    fn startup_policy_preserves_fatal_categories() {
        for code in [5, 6, 0xdead_beef] {
            let error = ApplyError::Register {
                hotkey: Hotkey::CtrlSpace,
                source: Source {
                    code,
                    is_win32: true,
                },
            };
            for phase in [StartupHotkeyPhase::Initial, StartupHotkeyPhase::Fallback] {
                assert_eq!(
                    classify_startup_error(phase, &error),
                    StartupHotkeyDecision::Fatal
                );
            }
            let ApplyError::Register { source, .. } = &error else {
                unreachable!("error is a register failure")
            };
            assert_eq!(source.code, code);
            assert!(source.is_win32);
        }

        let non_win32_1409 = ApplyError::Register {
            hotkey: Hotkey::CtrlSpace,
            source: Source {
                code: ERROR_HOTKEY_ALREADY_REGISTERED,
                is_win32: false,
            },
        };
        assert_eq!(
            classify_startup_error(StartupHotkeyPhase::Initial, &non_win32_1409),
            StartupHotkeyDecision::Fatal
        );
    }

    #[test]
    fn unregister_and_rollback_are_always_fatal() {
        let unregister = ApplyError::Unregister {
            hotkey: Hotkey::ShiftSpace,
            source: Source {
                code: ERROR_HOTKEY_ALREADY_REGISTERED,
                is_win32: true,
            },
        };
        let rollback = ApplyError::Rollback {
            operation: "register",
            hotkey: Hotkey::CtrlSpace,
            source: Source {
                code: ERROR_HOTKEY_ALREADY_REGISTERED,
                is_win32: true,
            },
        };
        assert_eq!(
            classify_startup_error(StartupHotkeyPhase::Initial, &unregister),
            StartupHotkeyDecision::Fatal
        );
        assert_eq!(
            classify_startup_error(StartupHotkeyPhase::Fallback, &rollback),
            StartupHotkeyDecision::Fatal
        );
    }
}
