use ksni::TrayService;

use crate::tray::KoffeeTray;

mod tray;
mod inhibitors;

fn main() {
    let service = TrayService::new(KoffeeTray {
        on: false
    });
    service.spawn();

    loop {
        std::thread::park();
    }
}
