use crate::platform::{NativeString, TrayError};
use crate::MenuItem;
use std::rc::Rc;

pub struct Tray {
    pub(crate) guid: u128,
    pub(crate) title: NativeString,
    pub(crate) click: Option<Rc<dyn Fn()>>,
    pub(crate) menu: Option<Vec<MenuItem>>,
    #[cfg(target_os = "linux")]
    pub(crate) icon: Option<Vec<ksni::Icon>>,
}

impl Tray {
    pub fn new(guid: u128, title: &str) -> Self {
        Self {
            guid,
            title: title.into(),
            click: None,
            menu: None,
            #[cfg(target_os = "linux")]
            icon: None,
        }
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

    #[cfg(target_os = "linux")]
    pub fn set_icon(&mut self, bytes: &[u8]) {
        let cursor = std::io::Cursor::new(bytes);
        if let Ok(ico) = ico::IconDir::read(cursor) {
            let entries = ico.entries();
            let mut icons = Vec::with_capacity(entries.len());

            for enrty in entries {
                if let Ok(img) = enrty.decode() {
                    icons.push(ksni::Icon {
                        width: img.width() as i32,
                        height: img.height() as i32,
                        data: img
                            .rgba_data()
                            .chunks_exact(4)
                            .flat_map(|v| [v[3], v[0], v[1], v[2]])
                            .collect(),
                    });
                }
            }

            self.icon = Some(icons)
        }
    }
}
