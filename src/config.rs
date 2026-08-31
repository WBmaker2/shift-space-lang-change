#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Hotkey {
    ShiftSpace,
    CtrlSpace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppSettings {
    pub shift_space_enabled: bool,
    pub ctrl_space_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsError {
    NoHotkeyEnabled,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            shift_space_enabled: true,
            ctrl_space_enabled: true,
        }
    }
}

impl AppSettings {
    pub fn new(shift_space_enabled: bool, ctrl_space_enabled: bool) -> Result<Self, SettingsError> {
        let settings = Self {
            shift_space_enabled,
            ctrl_space_enabled,
        };
        if settings.enabled_hotkeys().is_empty() {
            Err(SettingsError::NoHotkeyEnabled)
        } else {
            Ok(settings)
        }
    }

    pub fn with_hotkey(self, hotkey: Hotkey, enabled: bool) -> Result<Self, SettingsError> {
        let settings = match hotkey {
            Hotkey::ShiftSpace => Self {
                shift_space_enabled: enabled,
                ..self
            },
            Hotkey::CtrlSpace => Self {
                ctrl_space_enabled: enabled,
                ..self
            },
        };
        Self::new(settings.shift_space_enabled, settings.ctrl_space_enabled)
    }

    pub fn enabled_hotkeys(self) -> Vec<Hotkey> {
        let mut hotkeys = Vec::with_capacity(2);
        if self.shift_space_enabled {
            hotkeys.push(Hotkey::ShiftSpace);
        }
        if self.ctrl_space_enabled {
            hotkeys.push(Hotkey::CtrlSpace);
        }
        hotkeys
    }

    pub fn is_enabled(self, hotkey: Hotkey) -> bool {
        match hotkey {
            Hotkey::ShiftSpace => self.shift_space_enabled,
            Hotkey::CtrlSpace => self.ctrl_space_enabled,
        }
    }
}
