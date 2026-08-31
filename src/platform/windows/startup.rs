use std::path::{Path, PathBuf};

use crate::persistence::startup_command;
use crate::ports::StartupController;

use super::error::Win32Error;
use super::registry;

const RUN_SUBKEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
const RUN_VALUE: &str = "ShiftSpaceLangChange";

pub struct WinStartupController {
    executable: PathBuf,
}

impl WinStartupController {
    pub fn new(executable: impl AsRef<Path>) -> Self {
        Self {
            executable: executable.as_ref().to_path_buf(),
        }
    }
}

impl StartupController for WinStartupController {
    type Error = Win32Error;

    fn is_enabled(&self) -> Result<bool, Self::Error> {
        let key = registry::open_key(RUN_SUBKEY, registry::READ_ONLY)?;
        Ok(key
            .map(|key| registry::get_string(&key, RUN_VALUE).map(|value| value.is_some()))
            .transpose()?
            .unwrap_or(false))
    }

    fn set_enabled(&self, enabled: bool) -> Result<(), Self::Error> {
        if enabled {
            let key = registry::create_key(RUN_SUBKEY, registry::READ_WRITE)?;
            registry::set_string(&key, RUN_VALUE, &startup_command(&self.executable))
        } else if let Some(key) = registry::open_key(RUN_SUBKEY, registry::READ_WRITE)? {
            registry::delete_value(&key, RUN_VALUE)
        } else {
            Ok(())
        }
    }
}
