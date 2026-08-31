use std::path::Path;

use shift_space_lang_change::config::AppSettings;
use shift_space_lang_change::persistence::{decode_settings, startup_command};

#[test]
fn missing_or_invalid_values_recover_to_defaults() {
    assert_eq!(decode_settings(None, None).settings, AppSettings::default());
    assert!(decode_settings(None, None).needs_rewrite);
    assert_eq!(
        decode_settings(Some(7), Some(0)).settings,
        AppSettings::default()
    );
    assert_eq!(
        decode_settings(Some(0), Some(0)).settings,
        AppSettings::default()
    );
}

#[test]
fn startup_command_quotes_paths_with_spaces() {
    let path = Path::new(r"C:\Users\Kim User\App\shift-space-lang-change.exe");
    assert_eq!(
        startup_command(path),
        r#""C:\Users\Kim User\App\shift-space-lang-change.exe" --background"#
    );
}
