use anyhow::{bail, Result};
use std::env;
use std::fs;
use std::fs::create_dir_all;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

fn process_frame(input: &Path, output: &Path, lang: &str) -> Result<()> {
    let input = fs::read_to_string(input)?;
    let output_code = framec::run(&input, lang);
    fs::write(output, output_code)?;
    Ok(())
}

fn main() -> Result<()> {
    let input = PathBuf::from("src");
    if !input.is_dir() {
        bail!("{:?} isn't a directory", input);
    }

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    create_dir_all(&out)
        .unwrap_or_else(|_| panic!("Failed to create output directory: {:?}", &out));

    for entry in WalkDir::new(&input) {
        let entry = entry?;
        let path = entry.path();
        if path.extension().unwrap_or_default() == "frm" {
            println!("cargo:rerun-if-changed={:?}", &path);
            let stripped_path = path.strip_prefix(&input)?;
            let output_dir = match stripped_path.parent() {
                Some(parent) => out.join(parent),
                None => out.clone(),
            };
            create_dir_all(output_dir)?;

            let mut output_file = out.join(stripped_path);
            output_file.set_extension("rs");
            process_frame(&path, &output_file, "rust")?;
        }
    }
    Ok(())
}
