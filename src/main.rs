use std::env;

use anyhow::anyhow;
use anyhow::Result;
use clap::Parser;
use ksni::TrayService;
use log::info;

use crate::tray::Koffee;

mod inhibitors;
mod tray;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(debug_assertions)]
const DEV_MODE: bool = true;

#[cfg(not(debug_assertions))]
const DEV_MODE: bool = false;

#[derive(Parser)]
#[clap(version = VERSION)]
struct Options {
    #[clap(short, long)]
    inhibit: bool,
    #[clap(short, long)]
    light_mode: bool,
}

fn main() -> Result<()> {
    let in_shell = get_shell::get_shell().is_ok() || DEV_MODE;
    if in_shell {
        simple_logging::setup();
    } else {
        syslog_logging::setup();
        info!("Koffee-Tray {}", VERSION);
    }

    let options: Options = Options::parse();

    let koffee = tray::Koffee::new(options.light_mode);
    let service: TrayService<Koffee> = TrayService::new(koffee);
    let handle = service.handle();

    if options.inhibit {
        handle.update(tray::Koffee::switch);
    }

    service.run().map_err(|e| anyhow!(e))
}

mod syslog_logging {
    use log::LevelFilter;
    use syslog::{BasicLogger, Facility, Formatter3164};

    pub fn setup() {
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

mod simple_logging {
    use simple_logger::SimpleLogger;

    pub fn setup() {
        SimpleLogger::new().init().unwrap();
    }
}
