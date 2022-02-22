use anyhow::Result;
use frame_build::*;

fn main() -> Result<()> {
    FrameBuild::new().run()?;

    FrameBuild::new()
        .set_targets(&[TargetLanguage::Smcat])
        .include_only_if(move |path| {
            // the smcat backend panics if the state stack is used
            !path.to_str().unwrap().contains("stack")
        })
        .run()?;

    Ok(())
}
