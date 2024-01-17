use std::ptr::null;
use windows_sys::Win32::Foundation::{GetLastError, WIN32_ERROR};
use windows_sys::Win32::System::Diagnostics::Debug::{
    FormatMessageW, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS,
};

#[derive(Debug)]
pub struct WinError {
    code: WIN32_ERROR,
    message: String,
}

impl WinError {
    pub(super) fn new(code: WIN32_ERROR) -> Self {
        let message = unsafe { get_error_message(code) };
        Self { code, message }
    }

    pub(super) fn last() -> Self {
        Self::new(unsafe { GetLastError() })
    }

    pub fn code(&self) -> WIN32_ERROR {
        self.code
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

impl std::fmt::Display for WinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({:X})", self.message, self.code)
    }
}

impl std::error::Error for WinError {}

unsafe fn get_error_message(code: WIN32_ERROR) -> String {
    let mut buffer = [0u16; 4096];
    let length = FormatMessageW(
        FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
        null(),
        code,
        0,
        buffer.as_mut_ptr(),
        buffer.len() as u32,
        null(),
    );
    String::from_utf16_lossy(&buffer[..length as usize])
}
