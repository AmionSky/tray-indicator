use std::ptr::{null, null_mut};
use windows_sys::Win32::Foundation::{GetLastError, LocalFree, WIN32_ERROR};
use windows_sys::Win32::System::Diagnostics::Debug::{
    FormatMessageW, FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_FROM_SYSTEM,
    FORMAT_MESSAGE_IGNORE_INSERTS,
};

#[derive(Debug)]
pub struct WinError {
    code: WIN32_ERROR,
    message: String,
}

impl WinError {
    pub(super) fn last() -> Self {
        unsafe {
            let code = GetLastError();
            let message = get_error_message(code);
            Self { code, message }
        }
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
    let buffer: *mut u16 = null_mut();
    let length = FormatMessageW(
        FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
        null(),
        code,
        0,
        buffer,
        0,
        null(),
    );
    let message = unicode_to_string(buffer, length);
    LocalFree(buffer as _);
    message
}

unsafe fn unicode_to_string(ptr: *const u16, len: u32) -> String {
    let msg_native = std::slice::from_raw_parts(ptr, len as usize);
    String::from_utf16_lossy(msg_native)
}
