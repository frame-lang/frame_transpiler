use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::v3::CompilerV3;
use crate::frame_c::utils::RunError;
use std::path::{Path, PathBuf};

pub fn compile_multiple_bodies_demo<'a>(files: Vec<(&'a str, &'a str)>, lang: TargetLanguage) -> Result<Vec<(String, String)>, RunError> {
    let mut out = Vec::new();
    for (name, content) in files {
        let code = CompilerV3::compile_single_file(None, content, Some(lang), false)?;
        out.push((name.to_string(), code));
    }
    Ok(out)
}

pub fn compile_directory_demo(dir: &Path, lang: TargetLanguage, recursive: bool) -> Result<Vec<(PathBuf, String)>, RunError> {
    let mut outputs = Vec::new();
    fn walk(acc: &mut Vec<PathBuf>, p: &Path, recursive: bool) -> std::io::Result<()> {
        for entry in std::fs::read_dir(p)? {
            let entry = entry?; let path = entry.path();
            if path.is_dir() {
                if recursive { walk(acc, &path, recursive)?; }
            } else if path.is_file() {
                acc.push(path);
            }
        }
        Ok(())
    }
    let mut files = Vec::new();
    walk(&mut files, dir, recursive).map_err(|e| RunError::new(exitcode::IOERR, &format!("walk error: {}", e)))?;
    for file in files {
        match std::fs::read_to_string(&file) {
            Ok(content) => {
                let bytes = content.as_bytes();
                if bytes.first().copied() != Some(b'{') { continue; }
                match CompilerV3::compile_single_file(None, &content, Some(lang), false) {
                    Ok(code) => outputs.push((file.clone(), code)),
                    Err(e) => return Err(e),
                }
            }
            Err(_) => { /* skip unreadable */ }
        }
    }
    Ok(outputs)
}
