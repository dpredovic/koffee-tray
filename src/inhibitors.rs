use anyhow::Result;

pub trait Inhibitor {
    fn set_inhibit_state(&mut self, state: bool) -> Result<()>;
}

pub mod xdg {
    use anyhow::Result;
    use dbus::blocking::LocalConnection;

    pub struct Inhibitor {
        pub(crate) cookie: Option<u32>,
        conn: LocalConnection,
    }

    impl Inhibitor {
        pub(crate) fn new() -> Result<Self> {
            let connection = LocalConnection::new_session()?;
            Ok(Self {
                cookie: None,
                conn: connection,
            })
        }
    }

    pub mod power_management {
        use std::time::Duration;

        use anyhow::anyhow;
        use anyhow::Result;
        use log::info;

        use gen::OrgFreedesktopPowerManagementInhibit;

        #[allow(clippy::all)]
        mod gen {
            include!(concat!(env!("OUT_DIR"), "/xdg_power_management.rs"));
        }

        pub struct Inhibitor {
            xdg: super::Inhibitor,
        }

        impl Inhibitor {
            pub(crate) fn new() -> Result<Self> {
                Ok(Self {
                    xdg: super::Inhibitor::new()?,
                })
            }
        }

        impl crate::inhibitors::Inhibitor for Inhibitor {
            fn set_inhibit_state(&mut self, state: bool) -> Result<()> {
                let proxy = self.xdg.conn.with_proxy(
                    "org.freedesktop.PowerManagement.Inhibit",
                    "/org/freedesktop/PowerManagement/Inhibit",
                    Duration::from_millis(200),
                );
                if state {
                    let cookie = proxy.inhibit("Koffee-Tray", "Inhibition requested by user")?;
                    self.xdg.cookie = Some(cookie);
                    info!("xdg PowerManagement inhibited, cookie={cookie}");
                    Ok(())
                } else {
                    match self.xdg.cookie {
                        Some(cookie) => {
                            proxy.un_inhibit(cookie)?;
                            self.xdg.cookie = None;
                            info!("xdg PowerManagement uninhibited, cookie={cookie}");
                            Ok(())
                        }
                        None => Err(anyhow!("cookie is None!?")),
                    }
                }
            }
        }
    }

    pub mod screen_saver {
        use std::time::Duration;

        use anyhow::anyhow;
        use anyhow::Result;
        use log::info;

        use gen::OrgFreedesktopScreenSaver;

        #[allow(clippy::all)]
        mod gen {
            include!(concat!(env!("OUT_DIR"), "/xdg_screen_saver.rs"));
        }

        pub struct Inhibitor {
            xdg: super::Inhibitor,
        }

        impl Inhibitor {
            pub(crate) fn new() -> Result<Self> {
                Ok(Self {
                    xdg: super::Inhibitor::new()?,
                })
            }
        }

        impl crate::inhibitors::Inhibitor for Inhibitor {
            fn set_inhibit_state(&mut self, state: bool) -> Result<()> {
                let proxy = self.xdg.conn.with_proxy(
                    "org.freedesktop.ScreenSaver",
                    "/ScreenSaver",
                    Duration::from_millis(200),
                );
                if state {
                    let cookie = proxy.inhibit("Koffee-Tray", "Inhibition requested by user")?;
                    self.xdg.cookie = Some(cookie);
                    info!("xdg ScreenSaver inhibited, cookie={cookie}");
                    Ok(())
                } else {
                    match self.xdg.cookie {
                        Some(cookie) => {
                            proxy.un_inhibit(cookie)?;
                            self.xdg.cookie = None;
                            info!("xdg ScreenSaver uninhibited, cookie={cookie}");
                            Ok(())
                        }
                        None => Err(anyhow!("cookie is None?!")),
                    }
                }
            }
        }
    }
}
