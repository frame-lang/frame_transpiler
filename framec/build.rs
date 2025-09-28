use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Read version.toml from project root
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let project_root = Path::new(&manifest_dir).parent().unwrap();
    let version_file = project_root.join("version.toml");
    
    if version_file.exists() {
        let version_content = fs::read_to_string(&version_file)
            .expect("Failed to read version.toml");
        
        // Parse TOML (simple parsing for now)
        let full_version = extract_version_from_toml(&version_content);
        
        // Make version available as environment variable for compile time
        println!("cargo:rustc-env=FRAME_VERSION={}", full_version);
        
        // Re-run if version.toml changes
        println!("cargo:rerun-if-changed={}", version_file.display());
    } else {
        // Fallback to Cargo.toml version
        println!("cargo:rustc-env=FRAME_VERSION={}", env::var("CARGO_PKG_VERSION").unwrap());
    }
}

fn extract_version_from_toml(content: &str) -> String {
    // Simple TOML parsing for full = "x.y.z" line
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("full = \"") && line.ends_with("\"") {
            let start = line.find('"').unwrap() + 1;
            let end = line.rfind('"').unwrap();
            return line[start..end].to_string();
        }
    }
    "0.78.12".to_string() // fallback
}