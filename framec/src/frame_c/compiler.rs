use crate::frame_c::utils::RunError;
pub use crate::frame_c::visitors::TargetLanguage;
use exitcode;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

pub struct Exe {}

impl Exe {
    pub fn new() -> Exe {
        Exe {}
    }

    pub fn run_file(
        &self,
        input_path: &Path,
        target_language: Option<TargetLanguage>,
    ) -> Result<String, RunError> {
        match fs::read_to_string(input_path) {
            Ok(content) => {
                // Use v4 compiler for all Frame files
                // CLI -l flag takes precedence, fallback to @@target from file, then Python3
                let lang = target_language
                    .or_else(|| detect_at_target(&content))
                    .unwrap_or(TargetLanguage::Python3);
                let v4_lang = crate::frame_c::v4::TargetLanguage::from(lang);
                let compiler = crate::frame_c::v4::FrameV4Compiler::new(v4_lang);
                
                match compiler.compile(&content, input_path.to_str().unwrap_or("<unknown>")) {
                    crate::frame_c::v4::FrameV4Result::Ok(output) => Ok(output.code),
                    crate::frame_c::v4::FrameV4Result::Err(err) => {
                        let mut error_msg = String::from("Frame v4 compilation errors:\n");
                        for error in err.errors() {
                            error_msg.push_str(&format!("  {}\n", error));
                        }
                        Err(RunError::new(exitcode::DATAERR, &error_msg))
                    }
                }
            }
            Err(err) => Err(RunError::new(exitcode::NOINPUT, &format!("Cannot read file: {}", err))),
        }
    }

    pub fn run_file_debug(
        &self,
        input_path: &Path,
        target_language: Option<TargetLanguage>,
    ) -> Result<String, RunError> {
        match fs::read_to_string(input_path) {
            Ok(content) => {
                // Use v4 compiler for debug output
                // CLI -l flag takes precedence, fallback to @@target from file, then Python3
                let lang = target_language
                    .or_else(|| detect_at_target(&content))
                    .unwrap_or(TargetLanguage::Python3);
                let v4_lang = crate::frame_c::v4::TargetLanguage::from(lang);
                let compiler = crate::frame_c::v4::FrameV4Compiler::new(v4_lang);
                
                match compiler.compile(&content, input_path.to_str().unwrap_or("<unknown>")) {
                    crate::frame_c::v4::FrameV4Result::Ok(output) => Ok(output.code),
                    crate::frame_c::v4::FrameV4Result::Err(err) => {
                        let mut error_msg = String::from("Frame v4 compilation errors:\n");
                        for error in err.errors() {
                            error_msg.push_str(&format!("  {}\n", error));
                        }
                        Err(RunError::new(exitcode::DATAERR, &error_msg))
                    }
                }
            }
            Err(err) => Err(RunError::new(exitcode::NOINPUT, &format!("Cannot read file: {}", err))),
        }
    }

    pub fn run_multifile(
        &self,
        _entry_path: &Path,
        _target_language: Option<TargetLanguage>,
        _output_dir: Option<PathBuf>,
    ) -> Result<String, RunError> {
        Err(RunError::new(exitcode::USAGE, "Multi-file compilation not supported in V4"))
    }

    pub fn run_stdin(&self, target_language: Option<TargetLanguage>) -> Result<String, RunError> {
        let mut buffer = String::new();
        let mut stdin = io::stdin();
        match stdin.read_to_string(&mut buffer) {
            Ok(_size) => {
                // Use v4 compiler for stdin
                // CLI -l flag takes precedence, fallback to @@target from content, then Python3
                let lang = target_language
                    .or_else(|| detect_at_target(&buffer))
                    .unwrap_or(TargetLanguage::Python3);
                let v4_lang = crate::frame_c::v4::TargetLanguage::from(lang);
                let compiler = crate::frame_c::v4::FrameV4Compiler::new(v4_lang);
                
                match compiler.compile(&buffer, "<stdin>") {
                    crate::frame_c::v4::FrameV4Result::Ok(output) => Ok(output.code),
                    crate::frame_c::v4::FrameV4Result::Err(err) => {
                        let mut error_msg = String::from("Frame v4 compilation errors:\n");
                        for error in err.errors() {
                            error_msg.push_str(&format!("  {}\n", error));
                        }
                        Err(RunError::new(exitcode::DATAERR, &error_msg))
                    }
                }
            },
            Err(err) => Err(RunError::new(exitcode::NOINPUT, &format!("Cannot read stdin: {}", err))),
        }
    }
}

impl Default for Exe {
    fn default() -> Self {
        Exe::new()
    }
}

/// Detect @@target pragma in Frame V4 source files.
/// This is the primary target detection method for V4 files.
pub fn detect_at_target(content: &str) -> Option<TargetLanguage> {
    for line in content.lines() {
        let trimmed = line.trim();
        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with('#') {
            continue;
        }
        // Look for @@target <lang>
        if let Some(rest) = trimmed.strip_prefix("@@target") {
            let lang_str = rest.trim();
            // Extract the language token (first word)
            let lang_token = lang_str.split_whitespace().next()?.trim();
            return TargetLanguage::try_from(lang_token).ok();
        }
        // Stop looking after first non-comment, non-empty line that isn't @@target
        if !trimmed.starts_with("@@") {
            break;
        }
    }
    None
}

// Retained helpers for header target detection used by CLI in legacy workflows.
pub fn detect_header_target_annotation(content: &str) -> Option<TargetLanguage> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }
        if let Some(language) = parse_target_attribute(trimmed) {
            return Some(language);
        }
        if trimmed.starts_with('#') {
            continue;
        }
        break;
    }
    None
}

fn parse_target_attribute(line: &str) -> Option<TargetLanguage> {
    let inner = line.strip_prefix("#[")?.trim();
    let (body, _rest) = inner.split_once(']')?;
    let body = body.trim();
    let body = body.strip_prefix("target")?;
    let body = body.trim_start_matches(|c: char| c == ':' || c == '=' || c.is_whitespace());
    if body.is_empty() {
        return None;
    }
    let language_token = body.split_whitespace().next()?.trim();
    crate::frame_c::visitors::TargetLanguage::try_from(language_token).ok()
}
