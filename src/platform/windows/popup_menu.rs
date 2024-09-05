use super::{WinError, TRAY_DATA};
use crate::MenuItem;
use std::mem::size_of;
use std::ptr::{null, null_mut};
use thiserror::Error;
use windows_sys::core::PWSTR;
use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreatePopupMenu, DestroyMenu, InsertMenuItemW, SetForegroundWindow, TrackPopupMenuEx, HMENU,
    MENUITEMINFOW, MFS_GRAYED, MFT_SEPARATOR, MFT_STRING, MIIM_ID, MIIM_STATE, MIIM_TYPE,
    TPM_BOTTOMALIGN, TPM_LEFTALIGN, TPM_LEFTBUTTON, TRACK_POPUP_MENU_FLAGS,
};

const fn mii_default() -> MENUITEMINFOW {
    MENUITEMINFOW {
        cbSize: size_of::<MENUITEMINFOW>() as u32,
        fMask: 0,
        fType: 0,
        fState: 0,
        wID: 0,
        hSubMenu: null_mut(),
        hbmpChecked: null_mut(),
        hbmpUnchecked: null_mut(),
        dwItemData: 0,
        dwTypeData: null_mut(),
        cch: 0,
        hbmpItem: null_mut(),
    }
}

pub(super) unsafe fn display(hwnd: HWND, x: i32, y: i32) -> Result<(), MenuError> {
    // Create menu items
    let Some(items) = TRAY_DATA.with(|data| {
        data.borrow().menu.as_ref().map(|menu| {
            menu.iter()
                .enumerate()
                .map(|(i, item)| match item {
                    MenuItem::Button { label, action: _ } => MENUITEMINFOW {
                        fMask: MIIM_TYPE | MIIM_ID,
                        fType: MFT_STRING,
                        wID: i as u32,
                        dwTypeData: label.as_ptr() as PWSTR,
                        cch: label.len() - 1, // Does it need to be len or len-1?
                        ..mii_default()
                    },
                    MenuItem::Label { label } => MENUITEMINFOW {
                        fMask: MIIM_TYPE | MIIM_STATE,
                        fType: MFT_STRING,
                        fState: MFS_GRAYED,
                        dwTypeData: label.as_ptr() as PWSTR,
                        cch: label.len() - 1,
                        ..mii_default()
                    },
                    MenuItem::Separator => {
                        MENUITEMINFOW { fMask: MIIM_TYPE, fType: MFT_SEPARATOR, ..mii_default() }
                    }
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
        if InsertMenuItemW(menu.0, i as u32, 1, &item) == 0 {
            return Err(MenuError::AddItem(WinError::last()));
        }
    }

    if SetForegroundWindow(hwnd) == 0 {
        return Err(MenuError::WindowForground);
    }

    const FLAGS: TRACK_POPUP_MENU_FLAGS = TPM_LEFTALIGN | TPM_BOTTOMALIGN | TPM_LEFTBUTTON;
    if TrackPopupMenuEx(menu.0, FLAGS, x, y, hwnd, null()) == 0 {
        return Err(MenuError::Display(WinError::last()));
    }

    Ok(())
}

struct PopupMenu(HMENU);

impl PopupMenu {
    pub fn new() -> Result<Self, MenuError> {
        match unsafe { CreatePopupMenu() } {
            m if m.is_null() => Err(MenuError::Create(WinError::last())),
            menu => Ok(Self(menu)),
        }
    }
}

impl Drop for PopupMenu {
    fn drop(&mut self) {
        if unsafe { DestroyMenu(self.0) } == 0 {
            let error = WinError::last();
            eprintln!("Failed to destroy popup menu: {error}");
        }
    }
}

#[derive(Debug, Error)]
pub(super) enum MenuError {
    #[error("Failed to create popup menu. {0}")]
    Create(#[source] WinError),
    #[error("Failed to add menu item. {0}")]
    AddItem(#[source] WinError),
    #[error("Failed to bring dummy window to foreground.")]
    WindowForground,
    #[error("Failed to display popup menu. {0}")]
    Display(#[source] WinError),
}
