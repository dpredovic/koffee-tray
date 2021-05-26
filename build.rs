use std::env;
use std::process::Command;

fn main() -> Result<(), std::io::Error> {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("fish")
        .args(&[
            "-c",
            &format!(
                "dbus-codegen-rust -g \
                    -m None \
                    -d org.freedesktop.PowerManagement.Inhibit \
                    -p /org/freedesktop/PowerManagement/Inhibit \
                    -f org.freedesktop.PowerManagement.Inhibit \
                    -o {}/xdg_power_management.rs",
                out_dir
            ),
        ])
        .status()?;

    Command::new("fish")
        .args(&[
            "-c",
            &format!(
                "dbus-codegen-rust \
                    -g \
                    -m None \
                    -d org.freedesktop.ScreenSaver \
                    -p /ScreenSaver \
                    -f org.freedesktop.ScreenSaver \
                    -o {}/xdg_screen_saver.rs",
                out_dir
            ),
        ])
        .status()
        .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
