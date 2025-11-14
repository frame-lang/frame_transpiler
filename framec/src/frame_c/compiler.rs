use crate::frame_c::utils::RunError;
use crate::frame_c::v3::CompilerV3;
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
                // If this looks like a V3 module (has @target header), route to module compiler.
                // Prefer explicit CLI language, otherwise derive from header annotation.
                let header_lang = crate::frame_c::compiler::detect_header_target_annotation(&content);
                let chosen_lang = target_language.or(header_lang);
                if content.contains("@target ") {
                    // Require a target language to be known at this point
                    let lang = chosen_lang.unwrap_or(TargetLanguage::Python3);
                    return crate::frame_c::v3::compile_module_demo(&content, lang);
                }
                // Fallback to single-body demo compiler (expects body starting with '{')
                CompilerV3::compile_single_file(input_path.to_str(), &content, target_language, false)
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
                let header_lang = crate::frame_c::compiler::detect_header_target_annotation(&content);
                let chosen_lang = target_language.or(header_lang);
                if content.contains("@target ") {
                    let lang = chosen_lang.unwrap_or(TargetLanguage::Python3);
                    // In module path, debug trailers are controlled by env (set by CLI --emit-debug)
                    return crate::frame_c::v3::compile_module_demo(&content, lang);
                }
                CompilerV3::compile_single_file(input_path.to_str(), &content, target_language, true)
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
        CompilerV3::compile_multifile_unsupported()
    }

    pub fn run_stdin(&self, target_language: Option<TargetLanguage>) -> Result<String, RunError> {
        let mut buffer = String::new();
        let mut stdin = io::stdin();
        match stdin.read_to_string(&mut buffer) {
            Ok(_size) => CompilerV3::compile_single_file(None, &buffer, target_language, false),
            Err(err) => Err(RunError::new(exitcode::NOINPUT, &format!("Cannot read stdin: {}", err))),
        }
    }
}

impl Default for Exe {
    fn default() -> Self {
        Exe::new()
    }
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
