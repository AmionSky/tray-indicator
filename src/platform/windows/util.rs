use windows_sys::Win32::Foundation::{LPARAM, WPARAM};

pub(super) struct AsWord(u32);
impl From<LPARAM> for AsWord {
    fn from(value: LPARAM) -> Self {
        Self(value as u32)
    }
}
impl From<WPARAM> for AsWord {
    fn from(value: WPARAM) -> Self {
        Self(value as u32)
    }
}

#[inline]
pub(super) fn loword<T: Into<AsWord>>(val: T) -> u32 {
    val.into().0 & 0xffff
}

#[inline]
pub(super) fn hiword<T: Into<AsWord>>(val: T) -> u32 {
    (val.into().0 >> 16) & 0xffff
}

#[inline]
pub(super) fn get_x<T: Into<AsWord>>(val: T) -> i32 {
    loword(val) as i16 as i32
}

#[inline]
pub(super) fn get_y<T: Into<AsWord>>(val: T) -> i32 {
    hiword(val) as i16 as i32
}
