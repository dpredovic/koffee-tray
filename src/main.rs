use dbus::Error;
use ksni::TrayService;

use crate::inhibitors::dpms::DpmsInhibitor;
use crate::inhibitors::screen_saver::ScreenSaverInhibitor;
use crate::inhibitors::xdg_pm::XdgPowerManagement;
use crate::tray::KoffeeTray;

mod inhibitors;
mod tray;

fn main() -> Result<(), Error> {
    let service = TrayService::new(KoffeeTray {
        on: false,
        inhibitors: vec![
            Box::new(DpmsInhibitor),
            Box::new(ScreenSaverInhibitor),
            Box::new(XdgPowerManagement { cookie: None }),
        ],
    });
    service.run()
}
