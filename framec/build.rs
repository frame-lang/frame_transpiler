use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

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

    // Stage 14: FRM → Rust machine freshness advisory for IndentNormalizer.
    // Do not regenerate during build; instead, warn if the FRM has changed
    // more recently than its generated Rust counterpart.
    let indent_frs = project_root
        .join("framec")
        .join("src")
        .join("frame_c")
        .join("v3")
        .join("machines")
        .join("indent_normalizer.frs");
    let indent_gen_rs = project_root
        .join("framec")
        .join("src")
        .join("frame_c")
        .join("v3")
        .join("machines")
        .join("indent_normalizer.gen.rs");
    println!(
        "cargo:rerun-if-changed={}",
        indent_frs.display()
    );
    if indent_gen_rs.exists() {
        println!(
            "cargo:rerun-if-changed={}",
            indent_gen_rs.display()
        );
        if let (Ok(frs_meta), Ok(gen_meta)) =
            (fs::metadata(&indent_frs), fs::metadata(&indent_gen_rs))
        {
            let frs_time = frs_meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
            let gen_time = gen_meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
            if frs_time > gen_time {
                println!(
                    "cargo:warning=indent_normalizer.frs is newer than indent_normalizer.gen.rs; \
run tools/gen_v3_machines_rs.py with the bootstrap compiler (boot/framec/framec) to regenerate."
                );
            }
        }
    }
}
