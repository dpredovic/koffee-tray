use anyhow::anyhow;
use anyhow::Result;
use ksni::TrayService;
use log::info;

mod inhibitors;
mod tray;

fn main() -> Result<()> {
    logging::setup_logging();

    let service = TrayService::new(tray::Koffee {
        on: false,
        inhibitors: vec![
            Box::new(inhibitors::xdg::power_management::Inhibitor::new()?),
            Box::new(inhibitors::xdg::screen_saver::Inhibitor::new()?),
        ],
    });
    info!("Koffee-Tray service starting");
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
