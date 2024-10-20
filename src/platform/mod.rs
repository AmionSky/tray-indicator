mod error;
mod icon;
mod notify_icon;
mod popup_menu;
mod string;
mod util;

pub(crate) use string::NativeString;

use self::error::WinError;
use crate::Tray;
use std::cell::RefCell;

thread_local! {
    static TRAY_DATA: RefCell<Tray> = panic!("Tray data is not initialized");
}

pub(crate) fn display(tray: Tray) -> Result<(), TrayError> {
    TRAY_DATA.set(tray);

    unsafe {
        let instance = notify_icon::get_instance()?;
        let hwnd = notify_icon::create_window(instance)?;
        notify_icon::create_notify_icon(instance, hwnd)?;
        notify_icon::run_message_loop(hwnd);
    }

    Ok(())
}

pub(crate) fn exit() {
    unsafe {
        windows_sys::Win32::UI::WindowsAndMessaging::PostQuitMessage(0);
    }
}

#[derive(Debug)]
pub enum TrayError {
    NoInstance(WinError),
    WndClassRegister(WinError),
    WindowCreate(WinError),
    IconLoad(WinError),
    Display,
    Version,
}

impl std::fmt::Display for TrayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrayError::NoInstance(e) => write!(f, "Failed to get process instance handle. ({e})"),
            TrayError::WndClassRegister(e) => write!(f, "Failed to register window class. ({e})"),
            TrayError::WindowCreate(e) => write!(f, "Failed to create tray window. ({e})"),
            TrayError::IconLoad(e) => write!(f, "Failed to load icon for tray. ({e})"),
            TrayError::Display => write!(f, "Failed to display notify icon."),
            TrayError::Version => write!(f, "Notify icon API version is not supported by the OS."),
        }
    }
}

impl std::error::Error for TrayError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TrayError::NoInstance(e) => Some(e),
            TrayError::WndClassRegister(e) => Some(e),
            TrayError::WindowCreate(e) => Some(e),
            TrayError::IconLoad(e) => Some(e),
            TrayError::Display => None,
            TrayError::Version => None,
        }
    }
}
