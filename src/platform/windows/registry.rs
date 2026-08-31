use std::ffi::c_void;

use windows::Win32::Foundation::{
    ERROR_FILE_NOT_FOUND, ERROR_MORE_DATA, ERROR_SUCCESS, ERROR_UNSUPPORTED_TYPE,
};
use windows::Win32::System::Registry::{
    HKEY, HKEY_CURRENT_USER, KEY_READ, KEY_WRITE, REG_DWORD, REG_OPTION_NON_VOLATILE,
    REG_SAM_FLAGS, REG_SZ, REG_VALUE_TYPE, RRF_RT_REG_DWORD, RRF_RT_REG_SZ, RegCloseKey,
    RegCreateKeyExW, RegDeleteTreeW, RegDeleteValueW, RegGetValueW, RegOpenKeyExW, RegSetValueExW,
};
use windows::core::PCWSTR;

use super::error::Win32Error;

pub(crate) struct RegKey(HKEY);

impl RegKey {
    pub(crate) fn raw(&self) -> HKEY {
        self.0
    }
}

impl Drop for RegKey {
    fn drop(&mut self) {
        // Safety: this handle is returned by a successful registry open/create call and is
        // owned exclusively by this RAII wrapper until Drop.
        unsafe {
            let _ = RegCloseKey(self.0);
        }
    }
}

fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}

fn null_wide() -> PCWSTR {
    PCWSTR(std::ptr::null())
}

fn result(code: windows::Win32::Foundation::WIN32_ERROR) -> Result<(), Win32Error> {
    if code == ERROR_SUCCESS {
        Ok(())
    } else {
        Err(Win32Error::new(code.0))
    }
}

pub(crate) fn create_key(subkey: &str, access: REG_SAM_FLAGS) -> Result<RegKey, Win32Error> {
    let subkey = wide(subkey);
    let mut key = HKEY::default();
    // Safety: all pointers reference valid, NUL-terminated buffers for the duration of the call;
    // the output handle is an initialized writable slot and no security attributes are supplied.
    let code = unsafe {
        RegCreateKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(subkey.as_ptr()),
            None,
            null_wide(),
            REG_OPTION_NON_VOLATILE,
            access,
            None,
            &mut key,
            None,
        )
    };
    result(code).map(|()| RegKey(key))
}

pub(crate) fn open_key(subkey: &str, access: REG_SAM_FLAGS) -> Result<Option<RegKey>, Win32Error> {
    let subkey = wide(subkey);
    let mut key = HKEY::default();
    // Safety: the subkey buffer and output handle pointer are valid for this call; Windows only
    // writes the handle when it returns success.
    let code = unsafe {
        RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(subkey.as_ptr()),
            None,
            access,
            &mut key,
        )
    };
    if code == ERROR_FILE_NOT_FOUND {
        Ok(None)
    } else {
        result(code).map(|()| Some(RegKey(key)))
    }
}

pub(crate) fn get_dword(key: &RegKey, name: &str) -> Result<Option<u32>, Win32Error> {
    let name = wide(name);
    let mut value = 0_u32;
    let mut size = std::mem::size_of::<u32>() as u32;
    let mut value_type = REG_VALUE_TYPE(0);
    // Safety: value and size are writable buffers of the advertised size; the value-name buffer
    // is NUL-terminated and remains alive for the call.
    let code = unsafe {
        RegGetValueW(
            key.raw(),
            null_wide(),
            PCWSTR(name.as_ptr()),
            RRF_RT_REG_DWORD,
            Some(&mut value_type),
            Some((&mut value as *mut u32).cast::<c_void>()),
            Some(&mut size),
        )
    };
    if code == ERROR_FILE_NOT_FOUND || code == ERROR_MORE_DATA || code == ERROR_UNSUPPORTED_TYPE {
        Ok(None)
    } else {
        result(code)?;
        if value_type != REG_DWORD || size != std::mem::size_of::<u32>() as u32 {
            return Ok(None);
        }
        Ok(Some(value))
    }
}

pub(crate) fn set_dword(key: &RegKey, name: &str, value: u32) -> Result<(), Win32Error> {
    let name = wide(name);
    let bytes = value.to_le_bytes();
    // Safety: the value-name and four-byte data buffers remain valid and immutable for this call.
    let code = unsafe {
        RegSetValueExW(
            key.raw(),
            PCWSTR(name.as_ptr()),
            None,
            REG_DWORD,
            Some(&bytes),
        )
    };
    result(code)
}

pub(crate) fn get_string(key: &RegKey, name: &str) -> Result<Option<String>, Win32Error> {
    let name = wide(name);
    let mut value_type = REG_VALUE_TYPE(0);
    let mut size = 32_768_u32 * 2;
    let mut data = vec![0_u8; size as usize];
    // Safety: data is a writable buffer whose size is advertised in bytes, and both name and
    // output type buffers remain valid throughout this call.
    let code = unsafe {
        RegGetValueW(
            key.raw(),
            null_wide(),
            PCWSTR(name.as_ptr()),
            RRF_RT_REG_SZ,
            Some(&mut value_type),
            Some(data.as_mut_ptr().cast::<c_void>()),
            Some(&mut size),
        )
    };
    if code == ERROR_FILE_NOT_FOUND || code == ERROR_UNSUPPORTED_TYPE {
        return Ok(None);
    }
    result(code)?;
    data.truncate(size as usize);
    let units: Vec<u16> = data
        .chunks_exact(2)
        .map(|bytes| u16::from_le_bytes([bytes[0], bytes[1]]))
        .take_while(|&unit| unit != 0)
        .collect();
    Ok(Some(String::from_utf16_lossy(&units)))
}

pub(crate) fn set_string(key: &RegKey, name: &str, value: &str) -> Result<(), Win32Error> {
    let name = wide(name);
    let mut data = Vec::with_capacity(value.len() * 2 + 2);
    for unit in value.encode_utf16().chain(std::iter::once(0)) {
        data.extend_from_slice(&unit.to_le_bytes());
    }
    // Safety: the value-name and UTF-16 data buffers remain valid for this call, and the byte
    // count exactly describes the initialized data slice.
    let code =
        unsafe { RegSetValueExW(key.raw(), PCWSTR(name.as_ptr()), None, REG_SZ, Some(&data)) };
    result(code)
}

pub(crate) fn delete_value(key: &RegKey, name: &str) -> Result<(), Win32Error> {
    let name = wide(name);
    // Safety: the value-name buffer is valid and NUL-terminated for the duration of this call.
    let code = unsafe { RegDeleteValueW(key.raw(), PCWSTR(name.as_ptr())) };
    if code == ERROR_FILE_NOT_FOUND {
        Ok(())
    } else {
        result(code)
    }
}

pub(crate) fn delete_tree(subkey: &str) -> Result<(), Win32Error> {
    let subkey = wide(subkey);
    // Safety: the subkey buffer is valid and NUL-terminated for this call; HKCU is a predefined
    // root handle owned by Windows and must not be closed by this function.
    let code = unsafe { RegDeleteTreeW(HKEY_CURRENT_USER, PCWSTR(subkey.as_ptr())) };
    if code == ERROR_FILE_NOT_FOUND {
        Ok(())
    } else {
        result(code)
    }
}

pub(crate) const READ_WRITE: REG_SAM_FLAGS = REG_SAM_FLAGS(KEY_READ.0 | KEY_WRITE.0);
pub(crate) const READ_ONLY: REG_SAM_FLAGS = KEY_READ;
