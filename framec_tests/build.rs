use anyhow::Result;
use frame_build::*;
use std::path::PathBuf;

fn main() -> Result<()> {
    let targets = vec![TargetLanguage::Rust];
    frame_build(None, Config::new(), PathBuf::from("src"), targets)?;
    Ok(())
}
