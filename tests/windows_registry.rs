#![cfg(windows)]

use shift_space_lang_change::config::AppSettings;
use shift_space_lang_change::platform::windows::RegistrySettingsStore;
use shift_space_lang_change::ports::SettingsStore;

#[test]
fn settings_round_trip_uses_a_process_scoped_test_key() {
    let subkey = format!(
        "Software\\ShiftSpaceLangChange\\Tests\\{}",
        std::process::id()
    );
    let store = RegistrySettingsStore::with_subkey(subkey);
    let expected = AppSettings::new(true, false).expect("one hotkey must remain enabled");

    store.delete().expect("remove stale test key");
    store.save(expected).expect("save test settings");
    assert_eq!(store.load().expect("load test settings"), expected);
    store.delete().expect("remove test key");
}
