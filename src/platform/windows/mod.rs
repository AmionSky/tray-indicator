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
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum TrayError {
    #[error("Failed to get process instance handle. {0}")]
    NoInstance(#[source] WinError),
    #[error("Failed to register tray window class. {0}")]
    WndClassRegister(#[source] WinError),
    #[error("Failed to create tray window. {0}")]
    WindowCreate(#[source] WinError),
    #[error("Failed to load icon for tray. {0}")]
    IconLoad(#[source] WinError),
    #[error("Failed to display notify icon.")]
    Display,
    #[error("Notify icon API version is not supported by the OS.")]
    Version,
}
