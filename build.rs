use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let dest_path = Path::new("pkg");

    if !dest_path.exists() {
        fs::create_dir(dest_path).expect("Unable to create directory");
    }

    let test = Command::new("wasm-bindgen")
        .args(&[
            "target/wasm32-unknown-unknown/debug/rugl.wasm",
            "--out-dir",
            dest_path.to_str().unwrap(),
        ])
        .output()
        .expect("Unable to build wasm");

    println!("{:?}", test);
}
