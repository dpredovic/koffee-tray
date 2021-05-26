use ksni::menu::{CheckmarkItem, MenuItem, StandardItem};
use ksni::Tray;
use log::error;

use crate::inhibitors::Inhibitor;

pub struct Koffee {
    pub(crate) on: bool,
    pub(crate) inhibitors: Vec<Box<dyn Inhibitor>>,
}

impl Koffee {
    fn switch(&mut self) {
        self.on = !self.on;

        for i in &mut self.inhibitors {
            let result = i.set_inhibit_state(self.on);
            if let Err(err) = result {
                error!("error: {}", err)
            }
        }
    }
}

impl Tray for Koffee {
    fn activate(&mut self, _x: i32, _y: i32) {
        self.switch();
    }

    fn title(&self) -> String {
        if self.on {
            "Koffee is on".into()
        } else {
            "No koffee".into()
        }
    }

    fn icon_name(&self) -> String {
        if self.on {
            "user-available".into()
        } else {
            "user-offline".into()
        }
    }

    fn menu(&self) -> Vec<MenuItem<Self>> {
        vec![
            CheckmarkItem {
                label: "On".into(),
                enabled: true,
                visible: true,
                checked: self.on,
                activate: Box::new(Self::switch),
                ..ksni::menu::CheckmarkItem::default()
            }
            .into(),
            MenuItem::Sepatator,
            StandardItem {
                label: "Exit".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|_| std::process::exit(0)),
                ..ksni::menu::StandardItem::default()
            }
            .into(),
        ]
    }
}
