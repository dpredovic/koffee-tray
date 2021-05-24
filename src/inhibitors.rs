use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub trait Inhibitor {
    fn set_inhibit_state(&mut self, state: bool) -> Result<()>;
}

pub(crate) mod dpms {
    use std::process::Command;

    pub(crate) struct DpmsInhibitor;

    impl super::Inhibitor for DpmsInhibitor {
        fn set_inhibit_state(&mut self, state: bool) -> super::Result<()> {
            let arg = if state { "xset -dpms" } else { "xset +dpms" };
            let result = Command::new("sh").arg("-c").arg(arg).output()?;
            println!("Dpms set to {}: {}", state, result.status);
            Ok(())
        }
    }
}

pub(crate) mod screen_saver {
    use std::process::Command;

    pub struct ScreenSaverInhibitor;

    impl super::Inhibitor for ScreenSaverInhibitor {
        fn set_inhibit_state(&mut self, state: bool) -> super::Result<()> {
            let arg = if state { "xset s off" } else { "xset s on" };
            let result = Command::new("sh").arg("-c").arg(arg).output()?;
            println!("Screen saver set to {}: {}", state, result.status);
            Ok(())
        }
    }
}

pub(crate) mod xdg_pm {
    use std::time::Duration;

    use blocking::Proxy;
    use dbus::blocking;

    trait OrgFreedesktopPowerManagementInhibit {
        fn inhibit(&self, application: &str, reason: &str) -> Result<u32, dbus::Error>;
        fn un_inhibit(&self, cookie: u32) -> Result<(), dbus::Error>;
        fn has_inhibit(&self) -> Result<bool, dbus::Error>;
    }

    impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>>
        OrgFreedesktopPowerManagementInhibit for Proxy<'a, C>
    {
        fn inhibit(&self, application: &str, reason: &str) -> Result<u32, dbus::Error> {
            self.method_call(
                "org.freedesktop.PowerManagement.Inhibit",
                "Inhibit",
                (application, reason),
            )
            .and_then(|r: (u32,)| Ok(r.0))
        }

        fn un_inhibit(&self, cookie: u32) -> Result<(), dbus::Error> {
            self.method_call(
                "org.freedesktop.PowerManagement.Inhibit",
                "UnInhibit",
                (cookie,),
            )
        }

        fn has_inhibit(&self) -> Result<bool, dbus::Error> {
            self.method_call("org.freedesktop.PowerManagement.Inhibit", "HasInhibit", ())
                .and_then(|r: (bool,)| Ok(r.0))
        }
    }

    pub(crate) struct XdgPowerManagement {
        pub(crate) cookie: Option<u32>,
    }

    impl super::Inhibitor for XdgPowerManagement {
        fn set_inhibit_state(&mut self, state: bool) -> super::Result<()> {
            let conn = blocking::Connection::new_session()?;
            let proxy = conn.with_proxy(
                "org.freedesktop.PowerManagement",
                "/org/freedesktop/PowerManagement/Inhibit",
                Duration::from_millis(200),
            );
            if state {
                let cookie = proxy.inhibit("Koffee-Tray", "Inhibition requested by user")?;
                self.cookie = Some(cookie);
                println!("xdg pm inhibited, cookie={}", cookie)
            } else {
                let cookie = self.cookie.unwrap();
                proxy.un_inhibit(cookie)?;
                println!("xdg pm uninhibited, cookie={}", cookie)
            }
            Ok(())
        }
    }
}
