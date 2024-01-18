use super::error::WinError;
use windows_sys::core::PCWSTR;
use windows_sys::Win32::Foundation::HINSTANCE;
use windows_sys::Win32::UI::WindowsAndMessaging::{DestroyIcon, HICON};

pub(super) struct Icon(HICON);

impl Icon {
    #[cfg(not(feature = "dpiaware"))]
    pub fn new(instance: HINSTANCE, icon: PCWSTR) -> Result<Self, WinError> {
        use windows_sys::Win32::UI::WindowsAndMessaging::LoadIconW;

        match unsafe { LoadIconW(instance, icon) } {
            0 => Err(WinError::last()),
            handle => Ok(Self(handle)),
        }
    }

    #[cfg(feature = "dpiaware")]
    pub fn new(instance: HINSTANCE, icon: PCWSTR) -> Result<Self, WinError> {
        use windows_sys::Win32::Foundation::S_OK;
        use windows_sys::Win32::UI::Controls::{LoadIconMetric, LIM_SMALL};

        let mut handle = 0isize;
        match unsafe { LoadIconMetric(instance, icon, LIM_SMALL, &mut handle) } {
            S_OK => Ok(Self(handle)),
            error => Err(WinError::new(error as u32)),
        }
    }

    pub fn get(&self) -> HICON {
        self.0
    }
}

impl Drop for Icon {
    fn drop(&mut self) {
        unsafe { DestroyIcon(self.0) };
    }
}
