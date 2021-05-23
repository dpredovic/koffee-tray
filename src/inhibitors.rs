use std::process::Command;

pub trait Inhibitor {
    fn set_inhibit_state(state: bool);
}

pub enum DpmsInhibitor {}

impl Inhibitor for DpmsInhibitor {
    fn set_inhibit_state(state: bool) {
        let arg = if state { "xset -dpms" } else { "xset +dpms" };
        let result = Command::new("sh").arg("-c").arg(arg).output();
        match result {
            Ok(o) => { println!("Dpms set to {}: {}", state, o.status) }
            Err(err) => {
                println!("Error in Dpms inhibitor: {}", err);
            }
        }
    }
}

pub enum ScreenSaverInhibitor {}

impl Inhibitor for ScreenSaverInhibitor {
    fn set_inhibit_state(state: bool) {
        let arg = if state { "xset s off" } else { "xset s on" };
        let result = Command::new("sh").arg("-c").arg(arg).output();
        match result {
            Ok(o) => { println!("Screen saver set to {}: {}", state, o.status) }
            Err(err) => {
                println!("Error in Screen saver inhibitor: {}", err);
            }
        }
    }
}

