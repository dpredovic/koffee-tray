use ksni::TrayService;

use crate::tray::KoffeeTray;

mod inhibitors;
mod tray;

fn main() {
    let service = TrayService::new(KoffeeTray { on: false });
    service.spawn();

    loop {
        std::thread::park();
    }
}
