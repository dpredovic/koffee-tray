use std::process::Command;
use std::{env, fs};

use serde::Serialize;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let status = Command::new("fish")
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
        .status()
        .unwrap();
    assert!(status.success(), "status code = {}", status.code().unwrap());

    let status = Command::new("fish")
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
    assert!(status.success(), "status code = {}", status.code().unwrap());

    serialize_image("on_light", out_dir.as_ref());
    serialize_image("on_dark", out_dir.as_ref());
    serialize_image("off_light", out_dir.as_ref());
    serialize_image("off_dark", out_dir.as_ref());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/on_light.png");
    println!("cargo:rerun-if-changed=assets/on_dark.png");
    println!("cargo:rerun-if-changed=assets/off_light.png");
    println!("cargo:rerun-if-changed=assets/off_dark.png");
}

fn serialize_image(input: &str, out_dir: &str) {
    let image = image::open(format!("assets/{}.png", input)).unwrap();
    let image = image.as_rgba8().unwrap();
    let image = image.as_flat_samples();
    let image = ImageData {
        has_alpha: 1,
        data: image.samples,
    };
    let file = fs::File::create(format!("{}/{}.dbus", out_dir, input)).unwrap();
    bincode2::serialize_into(file, &image).unwrap();
}

#[derive(Serialize, Debug)]
struct ImageData<'a> {
    has_alpha: u8,
    data: &'a [u8],
}
