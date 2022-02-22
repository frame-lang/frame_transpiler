use anyhow::Result;
use frame_build::*;

fn main() -> Result<()> {
    let rust_config = Config::new();
    frame_build(rust_config)?;

    let smcat_config = Config::new()
        .set_targets(&[TargetLanguage::Smcat])
        .include_only_if(move |path| {
            // the smcat backend panics if the state stack is used
            !path.to_str().unwrap().contains("stack")
        });
    frame_build(smcat_config)?;

    Ok(())
}
