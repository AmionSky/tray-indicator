use std::iter::once;

pub(crate) struct NativeString(Vec<u16>);

impl NativeString {
    pub fn as_ptr(&self) -> *const u16 {
        self.0.as_ptr()
    }

    pub fn len(&self) -> u32 {
        self.0.len() as u32
    }
}

impl From<&str> for NativeString {
    fn from(value: &str) -> Self {
        Self(value.encode_utf16().chain(once(0u16)).collect())
    }
}

impl<const N: usize> From<&NativeString> for [u16; N] {
    fn from(value: &NativeString) -> Self {
        let mut array = [0u16; N];
        array[..value.0.len()].copy_from_slice(&value.0);
        array
    }
}
