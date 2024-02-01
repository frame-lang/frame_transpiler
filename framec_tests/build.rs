use anyhow::Result;
use frame_build::*;
use std::path::PathBuf;

struct Path {
    // golang: PathBuf,
    // javascript: PathBuf,
    python: PathBuf,
    // java: PathBuf,
}

impl Path {
    fn get_path() -> Path {
        Path {
            // golang: PathBuf::from("golang"),
            // javascript: PathBuf::from("javascript"),
            python: PathBuf::from("python"),
            // java: PathBuf::from("java"),
        }
    }
}

fn main() -> Result<()> {
    let path = Path::get_path();
    // FrameBuild::new().run()?;
    // FrameBuild::new()
    //     .set_targets(&[TargetLanguage::Smcat])
    //     .include_only_if(move |path| {
    //         // the smcat backend panics if the state stack is used
    //         !path.to_str().unwrap().contains("stack")
    //     })
    //     .run()?;

    // FrameBuild::new()
    //     .set_targets(&[TargetLanguage::GoLang])
    //     .input_dir(&path.golang)
    //     .output_dir(&path.golang)
    //     .run()?;
    // FrameBuild::new()
    //     .set_targets(&[TargetLanguage::JavaScript])
    //     .input_dir(&path.javascript)
    //     .output_dir(&path.javascript)
    //     .run()?;

    FrameBuild::new()
        .set_targets(&[TargetLanguage::Python3])
        .input_dir(&path.python)
        .output_dir(&path.python)
        .run()?;

    // FrameBuild::new()
    //     .set_targets(&[TargetLanguage::Java8])
    //     .input_dir(&path.java)
    //     .output_dir(&path.java)
    //     .run()?;
    //
    Ok(())
}
