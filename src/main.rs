use std::env;

use anyhow::anyhow;
use anyhow::Result;
use ksni::TrayService;
use log::info;

use crate::tray::Koffee;

mod inhibitors;
mod tray;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    logging::setup_logging();
    info!("Koffee-Tray v{}", VERSION);

    let args: Vec<String> = env::args().collect();

    let light = args.contains(&("-l".into()));

    let koffee = tray::Koffee::new(light);
    let service: TrayService<Koffee> = TrayService::new(koffee);
    let handle = service.handle();

    let on_startup = args.contains(&("-i".into()));
    if on_startup {
        handle.update(tray::Koffee::switch);
    }

    info!("Service starting");
    service.run().map_err(|e| anyhow!(e))
}

#[cfg(not(debug_assertions))]
mod logging {
    use log::LevelFilter;
    use syslog::{BasicLogger, Facility, Formatter3164};

    pub(crate) fn setup_logging() {
        let formatter = Formatter3164 {
            facility: Facility::LOG_USER,
            hostname: None,
            process: "Koffee-Tray".into(),
            pid: 0,
        };

        let logger = syslog::unix(formatter).expect("could not connect to syslog");
        log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
            .map(|()| log::set_max_level(LevelFilter::Info))
            .expect("could not set logger");
    }
}

#[cfg(debug_assertions)]
mod logging {
    use simple_logger::SimpleLogger;

    pub(crate) fn setup_logging() {
        SimpleLogger::new().init().unwrap();
    }
}
