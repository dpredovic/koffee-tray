use std::{env, fs};
use std::io::Write;
use std::process::Command;

use xcf::Xcf;

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

    for i in &["on_light", "on_dark", "off_light", "off_dark"] {
        serialize_image(i, out_dir.as_ref());
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/on_light.xcf");
    println!("cargo:rerun-if-changed=assets/on_dark.xcf");
    println!("cargo:rerun-if-changed=assets/off_light.xcf");
    println!("cargo:rerun-if-changed=assets/off_dark.xcf");
}

fn serialize_image(input: &str, out_dir: &str) {
    let xcf = Xcf::open(format!("assets/{}.xcf", input)).unwrap();
    assert_eq!(xcf.layers.len(), 1);
    let layer = &xcf.layers[0];
    assert_eq!(layer.width, 22);
    assert_eq!(layer.height, 22);
    let pixels = layer.raw_rgba_buffer().to_vec();
    assert_eq!(pixels.len(), 22 * 22);

    let mut file = fs::File::create(format!("{}/{}.dbus", out_dir, input)).unwrap();
    for pixel in pixels {
        file.write_all(&[pixel.a(), pixel.r(), pixel.g(), pixel.b()])
            .unwrap();
    }
}
