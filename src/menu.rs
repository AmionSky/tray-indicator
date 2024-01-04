use crate::platform::NativeString;

#[allow(private_interfaces)]
pub enum MenuItem {
    Button { label: NativeString, action: Box<dyn Fn()> },
    Label { label: NativeString },
    Separator,
}

impl MenuItem {
    pub fn button<F: 'static + Fn()>(label: &str, action: F) -> Self {
        Self::Button { label: label.into(), action: Box::new(action) }
    }

    pub fn label(label: &str) -> Self {
        Self::Label { label: label.into() }
    }

    pub fn separator() -> Self {
        Self::Separator
    }
}
