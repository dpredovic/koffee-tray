use ksni::menu::{CheckmarkItem, MenuItem, StandardItem};
use ksni::Tray;

use crate::inhibitors::Inhibitor;

pub(crate) struct KoffeeTray {
    pub(crate) on: bool,
    pub(crate) inhibitors: Vec<Box<dyn Inhibitor>>,
}

impl KoffeeTray {
    fn switch(&mut self) {
        self.on = !self.on;

        for i in self.inhibitors.iter_mut() {
            let result = i.set_inhibit_state(self.on);
            match result {
                Ok(_) => {}
                Err(err) => {
                    println!("error: {}", err)
                }
            }
        }
    }
}

impl Tray for KoffeeTray {
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
                activate: Box::new(|this: &mut Self| this.switch()),
                ..Default::default()
            }
            .into(),
            MenuItem::Sepatator,
            StandardItem {
                label: "Exit".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }
            .into(),
        ]
    }
}
