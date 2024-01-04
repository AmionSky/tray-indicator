use thiserror::Error;
use crate::Tray;

pub(crate) type NativeString = String;

pub(crate) fn display(tray: Tray) -> Result<(), TrayError> {
    todo!()
}

pub(crate) fn exit() {
    todo!()
}

#[derive(Debug, Error)]
pub enum TrayError {
    #[error("temperror")]
    Temp,
}