#[cfg_attr(target_os = "windows", path = "windows/mod.rs")]
#[cfg_attr(target_os = "linux", path = "linux/mod.rs")]
mod backend;

pub(crate) use backend::{display, exit, NativeString, TrayError};