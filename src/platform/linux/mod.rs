mod tray;

use crate::Tray;
use thiserror::Error;

pub(crate) type NativeString = String;

thread_local! {
    static HANDLE: std::cell::RefCell<Option<ksni::Handle<Tray>>>  = panic!("Tray handle is not initialized")
}

pub(crate) fn display(tray: Tray) -> Result<(), TrayError> {
    let service = ksni::TrayService::new(tray);
    HANDLE.set(Some(service.handle()));
    service.run().map_err(|_| TrayError::Dbus)?;
    Ok(())
}

pub(crate) fn exit() {
    if let Some(handle) = HANDLE.take() {
        handle.shutdown();
    }
}

#[derive(Debug, Error)]
pub enum TrayError {
    #[error("DBus error")]
    Dbus,
}
