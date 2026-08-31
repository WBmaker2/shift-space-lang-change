use crate::config::AppSettings;
use crate::persistence::decode_settings;
use crate::ports::SettingsStore;

use super::error::Win32Error;
use super::registry;

const PRODUCT_SUBKEY: &str = "Software\\ShiftSpaceLangChange";
const SHIFT_VALUE: &str = "ShiftSpaceEnabled";
const CONTROL_VALUE: &str = "CtrlSpaceEnabled";

pub struct RegistrySettingsStore {
    subkey: String,
}

impl RegistrySettingsStore {
    pub fn new() -> Self {
        Self::with_subkey(PRODUCT_SUBKEY.to_owned())
    }

    pub fn with_subkey(subkey: String) -> Self {
        Self { subkey }
    }

    pub fn delete(&self) -> Result<(), Win32Error> {
        registry::delete_tree(&self.subkey)
    }
}

impl Default for RegistrySettingsStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsStore for RegistrySettingsStore {
    type Error = Win32Error;

    fn load(&self) -> Result<AppSettings, Self::Error> {
        let values = registry::open_key(&self.subkey, registry::READ_ONLY)?;
        let (shift, control) = match values {
            Some(key) => (
                registry::get_dword(&key, SHIFT_VALUE)?,
                registry::get_dword(&key, CONTROL_VALUE)?,
            ),
            None => (None, None),
        };
        let decoded = decode_settings(shift, control);
        if decoded.needs_rewrite {
            self.save(decoded.settings)?;
        }
        Ok(decoded.settings)
    }

    fn save(&self, settings: AppSettings) -> Result<(), Self::Error> {
        let key = registry::create_key(&self.subkey, registry::READ_WRITE)?;
        registry::set_dword(
            &key,
            SHIFT_VALUE,
            u32::from(settings.is_enabled(crate::config::Hotkey::ShiftSpace)),
        )?;
        registry::set_dword(
            &key,
            CONTROL_VALUE,
            u32::from(settings.is_enabled(crate::config::Hotkey::CtrlSpace)),
        )
    }
}
