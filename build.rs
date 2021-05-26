use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("dbus-codegen-rust")
        .args(&[
            "-g",
            "-mNone",
            "-dorg.freedesktop.PowerManagement.Inhibit",
            "-p/org/freedesktop/PowerManagement/Inhibit",
            "-forg.freedesktop.PowerManagement.Inhibit",
            &format!("-o{}/xdg_power_management.rs", out_dir),
        ])
        .status()
        .unwrap();

    Command::new("dbus-codegen-rust")
        .args(&[
            "-g",
            "-mNone",
            "-dorg.freedesktop.ScreenSaver",
            "-p/ScreenSaver",
            "-forg.freedesktop.ScreenSaver",
            &format!("-o{}/xdg_screen_saver.rs", out_dir),
        ])
        .status()
        .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
