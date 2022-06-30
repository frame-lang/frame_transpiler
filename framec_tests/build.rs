use anyhow::Result;
use frame_build::*;
use std::path::PathBuf;

struct Path {
    golang: PathBuf,
    javascript: PathBuf,
}

impl Path {
    fn get_path() -> Path {
        Path {
            golang: PathBuf::from("golang"),
            javascript: PathBuf::from("javascript"),
        }
    }
}

fn main() -> Result<()> {
    let path = Path::get_path();
    FrameBuild::new().run()?;
    FrameBuild::new()
        .set_targets(&[TargetLanguage::Smcat])
        .include_only_if(move |path| {
            // the smcat backend panics if the state stack is used
            !path.to_str().unwrap().contains("stack")
        })
        .run()?;

    FrameBuild::new()
        .set_targets(&[TargetLanguage::GoLang])
        .input_dir(&path.golang)
        .output_dir(&path.golang)
        .run()?;
    FrameBuild::new()
        .set_targets(&[TargetLanguage::JavaScript])
        .input_dir(&path.javascript)
        .output_dir(&path.javascript)
        .run()?;

    Ok(())
}
