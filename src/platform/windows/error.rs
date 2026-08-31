use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Win32Error {
    code: u32,
    is_win32: bool,
}

impl Win32Error {
    pub const fn new(code: u32) -> Self {
        Self {
            code,
            is_win32: true,
        }
    }

    /// Preserve a non-Win32 HRESULT without allowing it to enter Win32 error classification.
    pub const fn from_hresult(code: u32) -> Self {
        Self {
            code,
            is_win32: false,
        }
    }

    pub const fn code(self) -> u32 {
        self.code
    }

    pub const fn is_win32(self) -> bool {
        self.is_win32
    }
}

impl fmt::Display for Win32Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Windows 오류 코드 {}", self.code)
    }
}

impl std::error::Error for Win32Error {}

impl crate::hotkeys::HotkeyErrorSource for Win32Error {
    fn code(&self) -> u32 {
        Win32Error::code(*self)
    }

    fn is_win32(&self) -> bool {
        Win32Error::is_win32(*self)
    }
}
