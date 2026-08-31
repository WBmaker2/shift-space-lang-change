use std::path::Path;

use crate::config::AppSettings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodedSettings {
    pub settings: AppSettings,
    pub needs_rewrite: bool,
}

/// Decode the two registry DWORDs, accepting only the canonical 0/1 values.
pub fn decode_settings(shift: Option<u32>, control: Option<u32>) -> DecodedSettings {
    let decoded = match (shift, control) {
        (Some(shift), Some(control)) if shift <= 1 && control <= 1 => {
            AppSettings::new(shift == 1, control == 1).ok()
        }
        _ => None,
    };

    match decoded {
        Some(settings) => DecodedSettings {
            settings,
            needs_rewrite: false,
        },
        None => DecodedSettings {
            settings: AppSettings::default(),
            needs_rewrite: true,
        },
    }
}

/// Build the command stored in the current user's Run registry value.
pub fn startup_command(executable: &Path) -> String {
    format!(r#""{}" --background"#, executable.display())
}
