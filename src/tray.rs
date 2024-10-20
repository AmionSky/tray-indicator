use crate::platform::{NativeString, TrayError};
use crate::MenuItem;
use std::rc::Rc;

pub struct Tray {
    pub(crate) guid: u128,
    pub(crate) title: NativeString,
    pub(crate) click: Option<Rc<dyn Fn()>>,
    pub(crate) menu: Option<Vec<MenuItem>>,
}

impl Tray {
    pub fn new(guid: u128, title: &str) -> Self {
        Self { guid, title: title.into(), click: None, menu: None }
    }

    pub fn set_click<F: 'static + Fn()>(&mut self, action: F) {
        self.click = Some(Rc::new(action));
    }

    pub fn set_menu(&mut self, menu: Vec<MenuItem>) {
        self.menu = Some(menu);
    }

    pub fn display(self) -> Result<(), TrayError> {
        crate::platform::display(self)
    }

    pub fn exit() {
        crate::platform::exit();
    }
}
