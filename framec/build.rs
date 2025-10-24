use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let frame_version =
        env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION must be set by Cargo");
    println!("cargo:rustc-env=FRAME_VERSION={}", frame_version);

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let project_root = Path::new(&manifest_dir).parent().unwrap();
    let version_file = project_root.join("version.toml");

    if version_file.exists() {
        if let Ok(contents) = fs::read_to_string(&version_file) {
            let expected = format!("full = \"{}\"", frame_version);
            if !contents.contains(&expected) {
                println!(
                    "cargo:warning=version.toml full version does not match workspace version ({})",
                    frame_version
                );
            }
        }
        println!("cargo:rerun-if-changed={}", version_file.display());
    }

    println!(
        "cargo:rerun-if-changed={}",
        project_root.join("Cargo.toml").display()
    );
}
