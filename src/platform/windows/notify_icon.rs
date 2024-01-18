use super::icon::Icon;
use super::{popup_menu, util, TrayError, WinError, TRAY_DATA};
use crate::MenuItem;
use std::cell::Cell;
use std::mem::size_of;
use std::ptr::null;
use windows_sys::core::{w, GUID, PCWSTR};
use windows_sys::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::Shell::{
    Shell_NotifyIconW, NIF_GUID, NIF_ICON, NIF_MESSAGE, NIF_SHOWTIP, NIF_TIP, NIM_ADD, NIM_MODIFY,
    NIM_SETVERSION, NIN_SELECT, NOTIFYICONDATAW, NOTIFYICONDATAW_0, NOTIFYICON_VERSION_4,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, PostQuitMessage,
    RegisterClassExW, RegisterWindowMessageW, TranslateMessage, CW_USEDEFAULT, IDI_APPLICATION,
    MSG, WM_COMMAND, WM_CONTEXTMENU, WM_CREATE, WM_DESTROY, WM_USER, WNDCLASSEXW, WS_EX_NOACTIVATE,
};

const TRAY_CLASS: PCWSTR = w!("TrayWndClass");
const WM_TRAY: u32 = WM_USER + 0x100;

pub(super) unsafe fn get_instance() -> Result<HINSTANCE, TrayError> {
    match GetModuleHandleW(null()) {
        0 => Err(TrayError::NoInstance(WinError::last())),
        instance => Ok(instance),
    }
}

pub(super) unsafe fn create_window(instance: HINSTANCE) -> Result<HWND, TrayError> {
    // Create and register dummy window class
    let class = WNDCLASSEXW {
        cbSize: size_of::<WNDCLASSEXW>() as u32,
        lpfnWndProc: Some(wndproc),
        hInstance: instance,
        lpszClassName: TRAY_CLASS,
        // Unset
        style: 0,
        cbClsExtra: 0,
        cbWndExtra: 0,
        hIcon: 0,
        hCursor: 0,
        hbrBackground: 0,
        lpszMenuName: null(),
        hIconSm: 0,
    };

    if RegisterClassExW(&class) == 0 {
        return Err(TrayError::WndClassRegister(WinError::last()));
    }

    // Create dummy window
    let hwnd = CreateWindowExW(
        WS_EX_NOACTIVATE,
        TRAY_CLASS,
        null(),
        0,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        0,
        0,
        instance,
        null(),
    );

    if hwnd == 0 {
        return Err(TrayError::WindowCreate(WinError::last()));
    }

    Ok(hwnd)
}

pub(super) unsafe fn create_notify_icon(instance: HINSTANCE, hwnd: HWND) -> Result<(), TrayError> {
    let icon = Icon::new(instance, IDI_APPLICATION).map_err(TrayError::IconLoad)?;

    let data = TRAY_DATA.with(|data| NOTIFYICONDATAW {
        cbSize: size_of::<NOTIFYICONDATAW>() as u32,
        hWnd: hwnd,
        guidItem: GUID::from_u128(data.borrow().guid),
        hIcon: icon.get(),
        uCallbackMessage: WM_TRAY,
        uFlags: NIF_GUID | NIF_SHOWTIP | NIF_TIP | NIF_ICON | NIF_MESSAGE,
        szTip: (&data.borrow().title).into(),
        Anonymous: NOTIFYICONDATAW_0 { uVersion: NOTIFYICON_VERSION_4 },
        // Unset
        uID: 0,
        dwState: 0,
        dwStateMask: 0,
        szInfo: [0; 256],
        szInfoTitle: [0; 64],
        dwInfoFlags: 0,
        hBalloonIcon: 0,
    });

    // Display / Update
    match Shell_NotifyIconW(NIM_ADD, &data) {
        // If it failed to add, try to modify it
        0 => {
            if Shell_NotifyIconW(NIM_MODIFY, &data) == 0 {
                return Err(TrayError::Display);
            }
        }
        // If it was successfully added, set API version
        _ => {
            if Shell_NotifyIconW(NIM_SETVERSION, &data) == 0 {
                return Err(TrayError::Version);
            }
        }
    }

    Ok(())
}

pub(super) unsafe fn run_message_loop(hwnd: HWND) {
    let mut msg =
        MSG { hwnd: 0, message: 0, wParam: 0, lParam: 0, time: 0, pt: POINT { x: 0, y: 0 } };

    while GetMessageW(&mut msg, hwnd, 0, 0) != 0 {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
}

unsafe fn recreate_notify_icon(hwnd: HWND) -> Result<(), TrayError> {
    let instance = get_instance()?;
    create_notify_icon(instance, hwnd)
}

thread_local! {
    static WM_TASKBAR_RESTART: Cell<u32> = Cell::new(0);
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        // Register taskbar restart message
        WM_CREATE => {
            WM_TASKBAR_RESTART.set(RegisterWindowMessageW(w!("TaskbarCreated")));
        }
        WM_TRAY => match util::loword(lparam) {
            NIN_SELECT => TRAY_DATA.with(|data| {
                if let Some(action) = &data.borrow().click {
                    action();
                }
            }),
            WM_CONTEXTMENU => {
                let x = util::get_x(wparam);
                let y = util::get_y(wparam);
                if let Err(error) = popup_menu::display(hwnd, x, y) {
                    eprintln!("Context menu error: {error}");
                }
            }
            _ => (),
        },
        WM_COMMAND => {
            if util::hiword(wparam) == 0 {
                TRAY_DATA.with(|data| {
                    let menu = &data.borrow().menu;
                    if let Some(menu) = menu {
                        let index = util::loword(wparam) as usize;
                        if let Some(MenuItem::Button { label: _, action }) = menu.get(index) {
                            action();
                        }
                    }
                });
            }
        }
        WM_DESTROY => PostQuitMessage(0),
        _ => {
            // Handle taskbar restart message
            if msg == WM_TASKBAR_RESTART.get() {
                if let Err(error) = recreate_notify_icon(hwnd) {
                    eprintln!("Failed to recreate tray icon: {error}");
                }
            }
        }
    }

    DefWindowProcW(hwnd, msg, wparam, lparam)
}
