#![cfg(windows)]

use shift_space_lang_change::config::AppSettings;
use shift_space_lang_change::platform::windows::RegistrySettingsStore;
use shift_space_lang_change::ports::SettingsStore;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry::{
    HKEY, HKEY_CURRENT_USER, KEY_WRITE, REG_DWORD, REG_OPTION_NON_VOLATILE, RegCloseKey,
    RegCreateKeyExW, RegSetValueExW,
};
use windows::core::PCWSTR;

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

#[test]
fn oversized_dword_is_rewritten_to_defaults() {
    let subkey = format!(
        "Software\\ShiftSpaceLangChange\\Tests\\{}-oversized",
        std::process::id()
    );
    let store = RegistrySettingsStore::with_subkey(subkey.clone());

    store.delete().expect("remove stale test key");
    store
        .save(AppSettings::default())
        .expect("save valid baseline");
    write_oversized_dword(&subkey, "ShiftSpaceEnabled");

    assert_eq!(
        store.load().expect("recover malformed settings"),
        AppSettings::default()
    );
    assert_eq!(
        store.load().expect("load rewritten settings"),
        AppSettings::default()
    );
    store.delete().expect("remove test key");
}

fn write_oversized_dword(subkey: &str, value_name: &str) {
    let subkey: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
    let value_name: Vec<u16> = value_name
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    let mut key = HKEY::default();
    // Safety: both UTF-16 buffers are NUL-terminated and live through the calls; key is a
    // writable output slot and the predefined HKCU handle is owned by Windows.
    let code = unsafe {
        RegCreateKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(subkey.as_ptr()),
            None,
            PCWSTR(std::ptr::null()),
            REG_OPTION_NON_VOLATILE,
            KEY_WRITE,
            None,
            &mut key,
            None,
        )
    };
    assert_eq!(code, ERROR_SUCCESS);

    let bytes = [1_u8, 0, 0, 0, 1, 0, 0, 0];
    // Safety: the value-name and eight-byte data buffers remain valid for this call, and key is
    // the handle returned by the successful create call above.
    let code = unsafe {
        RegSetValueExW(
            key,
            PCWSTR(value_name.as_ptr()),
            None,
            REG_DWORD,
            Some(&bytes),
        )
    };
    assert_eq!(code, ERROR_SUCCESS);
    // Safety: key was returned by RegCreateKeyExW and is closed exactly once here.
    let code = unsafe { RegCloseKey(key) };
    assert_eq!(code, ERROR_SUCCESS);
}
