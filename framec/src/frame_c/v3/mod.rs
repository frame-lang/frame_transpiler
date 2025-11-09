use crate::frame_c::utils::{frame_exitcode, RunError};
use crate::frame_c::visitors::TargetLanguage;

/// V3 compiler entrypoint (MVP scaffold).
///
/// This will replace the legacy pipeline incrementally. For now it returns a
/// deterministic error so the CLI remains usable while we bring up stages.
pub struct CompilerV3;

impl CompilerV3 {
    pub fn compile_single_file(
        _input_path: Option<&str>,
        _content: &str,
        _target_language: Option<TargetLanguage>,
        _debug_output: bool,
    ) -> Result<String, RunError> {
        Err(RunError::new(
            frame_exitcode::PARSE_ERR,
            "V3 pipeline not yet implemented — rebuilding from scratch (v3_architecture)",
        ))
    }

    pub fn compile_multifile_unsupported() -> Result<String, RunError> {
        Err(RunError::new(
            frame_exitcode::PARSE_ERR,
            "Multi-file build is temporarily unavailable during V3 rebuild",
        ))
    }
}

