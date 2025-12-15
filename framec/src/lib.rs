pub mod frame_c;
use crate::compiler::{Exe, TargetLanguage};
use crate::frame_c::*;
// CompilerV3 removed - using compile_module directly
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

/// Entry point for the online framepiler. See `frame_c::cli::run_with()` for
/// a more full-featured entry point.
#[wasm_bindgen]
pub fn run(frame_code: &str, format: &str) -> String {
    let _exe = Exe::new();
    match TargetLanguage::try_from(format) {
        Ok(target) => {
            if frame_code.contains("@target ") {
                let result = crate::frame_c::v3::compile_module(frame_code, target);
                match result {
                    Ok(code) => code,
                    Err(run_error) => {
                        // TODO: See about returning error code as well
                        run_error.error
                    }
                }
            } else {
                "Error: Frame files must specify @target language. Demo mode has been removed.".to_string()
            }
        }
        Err(err) => err,
    }
}
