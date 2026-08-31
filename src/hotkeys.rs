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
}

impl<B: HotkeyBackend> Drop for HotkeyManager<B> {
    fn drop(&mut self) {
        // Best-effort release keeps a partially initialized process from leaving registrations
        // behind. The Windows backend owns no Rust resources that need a second shutdown pass.
        for hotkey in self.active.enabled_hotkeys() {
            let _ = self.backend.unregister(hotkey);
        }
    }
}

impl<B: HotkeyBackend> HotkeyManager<B> {
    pub fn new(mut backend: B, active: AppSettings) -> Result<Self, ApplyError<B::Error>> {
        let mut registered = Vec::new();
        for hotkey in active.enabled_hotkeys() {
            if let Err(source) = backend.register(hotkey) {
                for registered_hotkey in registered.into_iter().rev() {
                    if let Err(rollback_source) = backend.unregister(registered_hotkey) {
                        return Err(ApplyError::Rollback {
                            operation: "unregister",
                            hotkey: registered_hotkey,
                            source: rollback_source,
                        });
                    }
                }
                return Err(ApplyError::Register { hotkey, source });
            }
            registered.push(hotkey);
        }
        Ok(Self { backend, active })
    }

    pub fn apply(&mut self, desired: AppSettings) -> Result<(), ApplyError<B::Error>> {
        let additions: Vec<_> = desired
            .enabled_hotkeys()
            .into_iter()
            .filter(|&hotkey| !self.active.is_enabled(hotkey))
            .collect();
        let removals: Vec<_> = self
            .active
            .enabled_hotkeys()
            .into_iter()
            .filter(|&hotkey| !desired.is_enabled(hotkey))
            .collect();

        let mut added = Vec::new();
        for hotkey in additions {
            if let Err(source) = self.backend.register(hotkey) {
                self.rollback_added(&added)?;
                return Err(ApplyError::Register { hotkey, source });
            }
            added.push(hotkey);
        }

        let mut removed = Vec::new();
        for hotkey in removals {
            if let Err(source) = self.backend.unregister(hotkey) {
                removed.push(hotkey);
                self.rollback(&added, &removed)?;
                return Err(ApplyError::Unregister { hotkey, source });
            }
            removed.push(hotkey);
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

    fn rollback_added(&mut self, added: &[Hotkey]) -> Result<(), ApplyError<B::Error>> {
        for &hotkey in added.iter().rev() {
            self.backend
                .unregister(hotkey)
                .map_err(|source| ApplyError::Rollback {
                    operation: "unregister",
                    hotkey,
                    source,
                })?;
        }
        Ok(())
    }

    fn rollback(
        &mut self,
        added: &[Hotkey],
        removed: &[Hotkey],
    ) -> Result<(), ApplyError<B::Error>> {
        for &hotkey in removed.iter().rev() {
            self.backend
                .register(hotkey)
                .map_err(|source| ApplyError::Rollback {
                    operation: "register",
                    hotkey,
                    source,
                })?;
        }
        self.rollback_added(added)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{ERROR_HOTKEY_ALREADY_REGISTERED, HotkeyErrorClass, classify_hotkey_error};

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
}
