use super::TRAY_DATA;
use crate::MenuItem;
use std::mem::size_of;
use thiserror::Error;
use windows::core::PWSTR;
use windows::Win32::Foundation::{GetLastError, HWND};
use windows::Win32::UI::WindowsAndMessaging::{
    CreatePopupMenu, DestroyMenu, InsertMenuItemW, SetForegroundWindow, TrackPopupMenuEx, HMENU,
    MENUITEMINFOW, MFS_GRAYED, MFT_SEPARATOR, MFT_STRING, MIIM_ID, MIIM_STATE, MIIM_TYPE,
    TPM_BOTTOMALIGN, TPM_LEFTALIGN, TPM_LEFTBUTTON,
};

pub(super) unsafe fn display(hwnd: HWND, x: i32, y: i32) -> Result<(), MenuError> {
    // Create menu items
    let Some(items) = TRAY_DATA.with(|data| {
        data.borrow().menu.as_ref().map(|menu| {
            menu.iter()
                .enumerate()
                .map(|(i, item)| match item {
                    MenuItem::Button { label, action: _ } => MENUITEMINFOW {
                        cbSize: size_of::<MENUITEMINFOW>() as u32,
                        fMask: MIIM_TYPE | MIIM_ID,
                        fType: MFT_STRING,
                        wID: i as u32,
                        dwTypeData: PWSTR::from_raw(label.as_ptr() as *mut _),
                        cch: label.len() - 1, // Does it need to be len or len-1?
                        ..Default::default()
                    },
                    MenuItem::Text { label } => MENUITEMINFOW {
                        cbSize: size_of::<MENUITEMINFOW>() as u32,
                        fMask: MIIM_TYPE | MIIM_STATE,
                        fType: MFT_STRING,
                        fState: MFS_GRAYED,
                        dwTypeData: PWSTR::from_raw(label.as_ptr() as *mut _),
                        cch: label.len() - 1,
                        ..Default::default()
                    },
                    MenuItem::Separator => MENUITEMINFOW {
                        cbSize: size_of::<MENUITEMINFOW>() as u32,
                        fMask: MIIM_TYPE,
                        fType: MFT_SEPARATOR,
                        ..Default::default()
                    },
                })
                .collect::<Vec<MENUITEMINFOW>>()
        })
    }) else {
        // No menu was specified
        return Ok(());
    };

    let menu = PopupMenu::new()?;

    // Add menu items
    for (i, item) in items.into_iter().enumerate() {
        InsertMenuItemW(menu.0, i as u32, true, &item).map_err(MenuError::AddItem)?;
    }

    if !SetForegroundWindow(hwnd).as_bool() {
        return Err(MenuError::WindowForground);
    }

    let result = TrackPopupMenuEx(
        menu.0,
        TPM_LEFTALIGN.0 | TPM_BOTTOMALIGN.0 | TPM_LEFTBUTTON.0,
        x,
        y,
        hwnd,
        None,
    );

    if !result.as_bool() {
        if let Err(error) = GetLastError() {
            return Err(MenuError::Display(error));
        }
    }

    Ok(())
}

struct PopupMenu(HMENU);

impl PopupMenu {
    pub fn new() -> Result<Self, MenuError> {
        match unsafe { CreatePopupMenu() } {
            Ok(menu) => Ok(Self(menu)),
            Err(error) => Err(MenuError::Create(error)),
        }
    }
}

impl Drop for PopupMenu {
    fn drop(&mut self) {
        if let Err(error) = unsafe { DestroyMenu(self.0) } {
            eprintln!("Failed to destroy popup menu: {error}");
        }
    }
}

#[derive(Debug, Error)]
pub(super) enum MenuError {
    #[error("Failed to create popup menu. {0}")]
    Create(#[source] windows::core::Error),
    #[error("Failed to add menu item. {0}")]
    AddItem(#[source] windows::core::Error),
    #[error("Failed to bring dummy window to foreground.")]
    WindowForground,
    #[error("Failed to display popup menu. {0}")]
    Display(#[source] windows::core::Error),
}
