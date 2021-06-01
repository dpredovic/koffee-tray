use ksni::menu::{CheckmarkItem, MenuItem, StandardItem};
use ksni::{Icon, Tray};
use log::error;
use rust_embed::RustEmbed;

use crate::inhibitors;
use crate::inhibitors::Inhibitor;

#[derive(RustEmbed)]
#[folder = "$OUT_DIR"]
struct Asset;

pub struct Koffee {
    pub(crate) on: bool,
    pub(crate) inhibitors: Vec<Box<dyn Inhibitor>>,
    light_mode: bool,
}

impl Koffee {
    pub(crate) fn new(light: bool) -> Self {
        Self {
            on: false,
            inhibitors: vec![
                Box::new(inhibitors::xdg::power_management::Inhibitor::new().unwrap()),
                Box::new(inhibitors::xdg::screen_saver::Inhibitor::new().unwrap()),
            ],
            light_mode: light,
        }
    }

    pub fn switch(&mut self) {
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
            "Koffee".into()
        } else {
            "No koffee".into()
        }
    }

    fn icon_pixmap(&self) -> Vec<Icon> {
        let prefix = if self.on { "on" } else { "off" };
        let suffix = if self.light_mode { "_light" } else { "_dark" };
        let name = format!("{}{}.dbus", prefix, suffix);

        vec![Icon {
            width: 22,
            height: 22,
            data: Asset::get(name.as_ref()).unwrap().to_vec(),
        }]
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
