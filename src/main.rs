mod dpms_inhibitor;
mod screen_saver_inhibitor;

use ksni;
use ksni::{MenuItem, TrayService, Tray};
use ksni::menu::{CheckmarkItem, StandardItem};
use crate::dpms_inhibitor::DpmsInhibitor;
use crate::screen_saver_inhibitor::ScreenSaverInhibitor;

struct KoffeeTray {
    on: bool,
}

impl KoffeeTray {
    fn switch(&mut self) {
        self.on = !self.on;

        ScreenSaverInhibitor::set_inhibit_state(self.on);
        DpmsInhibitor::set_inhibit_state(self.on);
    }
}

pub trait Inhibitor {
    fn set_inhibit_state(state: bool);
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
            }.into(),
            MenuItem::Sepatator,
            StandardItem {
                label: "Exit".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }.into(),
        ]
    }
}

fn main() {
    let service = TrayService::new(KoffeeTray {
        on: false
    });
    service.spawn();

    loop {
        std::thread::park();
    }
}
