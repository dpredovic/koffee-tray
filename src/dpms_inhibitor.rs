use std::process::Command;

use crate::Inhibitor;

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
