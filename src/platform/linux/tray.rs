impl ksni::Tray for crate::Tray {
    fn id(&self) -> String {
        self.guid.to_string()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn category(&self) -> ksni::Category {
        ksni::Category::ApplicationStatus
    }

    fn activate(&mut self, _x: i32, _y: i32) {
        if let Some(action) = &self.click {
            action();
        }
    }

    fn icon_pixmap(&self) -> Vec<ksni::Icon> {
        self.icon.clone().unwrap_or_default()
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use crate::MenuItem;

        if let Some(menu) = &self.menu {
            return menu
                .iter()
                .map(|item| match item {
                    MenuItem::Button { label, action } => {
                        let action = action.clone();
                        ksni::menu::MenuItem::Standard(ksni::menu::StandardItem {
                            label: label.clone(),
                            enabled: true,
                            visible: true,
                            activate: Box::new(move |_| {
                                action();
                            }),
                            ..Default::default()
                        })
                    }
                    MenuItem::Label { label } => {
                        ksni::menu::MenuItem::Standard(ksni::menu::StandardItem {
                            label: label.clone(),
                            enabled: false,
                            visible: true,
                            ..Default::default()
                        })
                    }
                    MenuItem::Separator => ksni::menu::MenuItem::Separator,
                })
                .collect();
        }

        Default::default()
    }

    fn watcher_offine(&self) -> bool {
        false
    }
}
