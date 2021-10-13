use anyhow::{bail, Result};
use framec::frame_c::compiler::Exe;
use std::env;
use std::fs;
use std::fs::create_dir_all;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

fn process_frame(input_path: &Path, output_path: &Path, lang: &str) -> Result<()> {
    let exe = Exe::new();
    let output_code = exe.run_file(&None, input_path, lang.to_string())?;
    fs::write(output_path, output_code)?;
    Ok(())
}

fn main() -> Result<()> {
    let input_dir = PathBuf::from("src");
    if !input_dir.is_dir() {
        bail!("{:?} isn't a directory", input_dir);
    }
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    create_dir_all(&out)
        .unwrap_or_else(|_| panic!("Failed to create output directory: {:?}", &out));

    for entry in WalkDir::new(&input_dir) {
        let entry = entry?;
        let input_path = entry.path();
        if input_path.extension().unwrap_or_default() == "frm" {
            println!("cargo:rerun-if-changed={:?}", &input_path);
            let stripped_path = input_path.strip_prefix(&input_dir)?;
            let output_dir = match stripped_path.parent() {
                Some(parent) => out.join(parent),
                None => out.clone(),
            };
            create_dir_all(output_dir)?;

            let mut output_file = out.join(stripped_path);
            output_file.set_extension("rs");
            process_frame(input_path, &output_file, "rust")?;
        }
    }
    Ok(())
}
