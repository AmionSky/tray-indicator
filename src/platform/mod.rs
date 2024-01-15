#[cfg_attr(target_os = "windows", path = "windows/mod.rs")]
#[cfg_attr(target_os = "linux", path = "linux/mod.rs")]
mod backend;

pub use backend::TrayError;
pub(crate) use backend::{display, exit, NativeString};
