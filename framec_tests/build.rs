use anyhow::{bail, Result};
use framec::frame_c::compiler::{Exe, TargetLanguage};
use std::env;
use std::fs;
use std::fs::create_dir_all;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

fn process_frame(input_path: &Path, output_path: &Path, lang: TargetLanguage) -> Result<()> {
    let exe = Exe::new();
    let output_code = exe.run_file(&None, input_path, Some(lang))?;
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

            let mut rust_output_path = out.join(stripped_path);
            let mut smcat_output_path = rust_output_path.clone();
            rust_output_path.set_extension("rs");
            smcat_output_path.set_extension("smcat");
            process_frame(input_path, &rust_output_path, TargetLanguage::Rust)?;
            let smcat_catch = std::panic::catch_unwind(|| {
                let smcat_result =
                    process_frame(input_path, &smcat_output_path, TargetLanguage::Smcat);
                if let Err(err) = smcat_result {
                    eprintln!(
                        "smcat generation for {:?} returned an error: {:?}",
                        input_path, err
                    );
                }
            });
            if let Err(cause) = smcat_catch {
                eprintln!(
                    "smcat generation for {:?} panicked: {:?}",
                    input_path, cause
                );
            }
        }
    }
    Ok(())
}
