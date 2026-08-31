use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Win32Error {
    code: u32,
}

impl Win32Error {
    pub const fn new(code: u32) -> Self {
        Self { code }
    }

    pub const fn code(self) -> u32 {
        self.code
    }
}

impl fmt::Display for Win32Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Windows 오류 코드 {}", self.code)
    }
}

impl std::error::Error for Win32Error {}
