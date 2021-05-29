use std::env;

use anyhow::anyhow;
use anyhow::Result;
use clap::Clap;
use ksni::TrayService;
use log::info;

use crate::tray::Koffee;

mod inhibitors;
mod tray;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clap)]
#[clap(version = VERSION)]
struct Options {
    #[clap(short, long)]
    inhibit: bool,
    #[clap(short, long)]
    light_mode: bool,
}

fn main() -> Result<()> {
    logging::setup_logging();

    #[cfg(not(debug_assertions))]
    info!("Koffee-Tray {}", VERSION);

    let options: Options = Options::parse();

    let koffee = tray::Koffee::new(options.light_mode);
    let service: TrayService<Koffee> = TrayService::new(koffee);
    let handle = service.handle();

    if options.inhibit {
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
