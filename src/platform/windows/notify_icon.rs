use super::popup_menu;
use super::{TrayError, TRAY_DATA};
use crate::MenuItem;
use std::mem::size_of;
use windows::core::{w, GUID, PCWSTR};
use windows::Win32::Foundation::{GetLastError, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Shell::{
    Shell_NotifyIconW, NIF_GUID, NIF_ICON, NIF_MESSAGE, NIF_SHOWTIP, NIF_TIP, NIM_ADD,
    NIM_SETVERSION, NIN_SELECT, NOTIFYICONDATAW, NOTIFYICONDATAW_0, NOTIFYICON_VERSION_4,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, LoadIconW, PostQuitMessage,
    RegisterClassExW, TranslateMessage, CW_USEDEFAULT, IDI_APPLICATION, MSG, WINDOW_STYLE,
    WM_COMMAND, WM_CONTEXTMENU, WM_DESTROY, WM_USER, WNDCLASSEXW, WS_EX_NOACTIVATE,
};

const TRAY_CLASS: PCWSTR = w!("TrayWndClass");
const WM_TRAY: u32 = WM_USER + 0x100;

pub(super) unsafe fn get_instance() -> Result<HINSTANCE, TrayError> {
    Ok(GetModuleHandleW(None).map_err(TrayError::NoInstance)?.into())
}

pub(super) unsafe fn create_window(instance: HINSTANCE) -> Result<HWND, TrayError> {
    // Create and register dummy window class
    let class = WNDCLASSEXW {
        cbSize: size_of::<WNDCLASSEXW>() as u32,
        lpfnWndProc: Some(wndproc),
        hInstance: instance,
        lpszClassName: TRAY_CLASS,
        ..Default::default()
    };

    if RegisterClassExW(&class) == 0 {
        if let Err(error) = GetLastError() {
            return Err(TrayError::WndClassRegister(error));
        }
    }

    // Create dummy window
    let hwnd = CreateWindowExW(
        WS_EX_NOACTIVATE,
        TRAY_CLASS,
        None,
        WINDOW_STYLE(0),
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        None,
        None,
        instance,
        None,
    );

    if hwnd.0 == 0 {
        if let Err(error) = GetLastError() {
            return Err(TrayError::WindowCreate(error));
        }
    }

    Ok(hwnd)
}

pub(super) unsafe fn create_notify_icon(instance: HINSTANCE, hwnd: HWND) -> Result<(), TrayError> {
    let icon = LoadIconW(instance, IDI_APPLICATION).map_err(TrayError::IconLoad)?; // LoadIconMetric ?
    let data = TRAY_DATA.with(|data| NOTIFYICONDATAW {
        cbSize: size_of::<NOTIFYICONDATAW>() as u32,
        hWnd: hwnd,
        guidItem: GUID::from_u128(data.borrow().guid),
        hIcon: icon,
        uCallbackMessage: WM_TRAY,
        uFlags: NIF_GUID | NIF_SHOWTIP | NIF_TIP | NIF_ICON | NIF_MESSAGE,
        szTip: (&data.borrow().title).into(),
        Anonymous: NOTIFYICONDATAW_0 { uVersion: NOTIFYICON_VERSION_4 },
        ..Default::default()
    });

    // Display
    if !Shell_NotifyIconW(NIM_ADD, &data).as_bool() {
        return Err(TrayError::Display);
    }

    // Set API version
    if !Shell_NotifyIconW(NIM_SETVERSION, &data).as_bool() {
        return Err(TrayError::Version);
    }

    Ok(())
}

pub(super) unsafe fn run_message_loop(hwnd: HWND) {
    let mut msg = MSG::default();
    while GetMessageW(&mut msg, hwnd, 0, 0).as_bool() {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_TRAY => match loword(lparam) {
            NIN_SELECT => TRAY_DATA.with(|data| {
                if let Some(action) = &data.borrow().click {
                    action();
                }
            }),
            WM_CONTEXTMENU => {
                if let Err(error) = popup_menu::display(hwnd, get_x(wparam), get_y(wparam)) {
                    eprintln!("Context menu error: {error}");
                }
            }
            _ => (),
        },
        WM_COMMAND => {
            if hiword(wparam) == 0 {
                TRAY_DATA.with(|data| {
                    let menu = &data.borrow().menu;
                    if let Some(menu) = menu {
                        let index = loword(wparam) as usize;
                        if let Some(MenuItem::Button { label: _, action }) = menu.get(index) {
                            action();
                        }
                    }
                });
            }
        }
        WM_DESTROY => PostQuitMessage(0),
        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    }
    LRESULT(0)
}

struct AsWord(u32);
impl From<LPARAM> for AsWord {
    fn from(value: LPARAM) -> Self {
        Self(value.0 as u32)
    }
}
impl From<WPARAM> for AsWord {
    fn from(value: WPARAM) -> Self {
        Self(value.0 as u32)
    }
}

#[inline]
fn loword<T: Into<AsWord>>(val: T) -> u32 {
    val.into().0 & 0xffff
}

#[inline]
fn hiword<T: Into<AsWord>>(val: T) -> u32 {
    (val.into().0 >> 16) & 0xffff
}

#[inline]
fn get_x<T: Into<AsWord>>(val: T) -> i32 {
    loword(val) as i16 as i32
}

#[inline]
fn get_y<T: Into<AsWord>>(val: T) -> i32 {
    hiword(val) as i16 as i32
}
