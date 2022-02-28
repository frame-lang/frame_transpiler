pub mod frame_c;
use crate::compiler::{Exe, TargetLanguage};
use crate::frame_c::*;
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

/// Entry point for the online framepiler. See `frame_c::cli::run_with()` for
/// a more full-featured entry point.
#[wasm_bindgen]
pub fn run(frame_code: &str, format: &str) -> String {
    let exe = Exe::new();
    match TargetLanguage::try_from(format) {
        Ok(target) => {
            let result = exe.run(&None, None, frame_code.to_string(), Some(target));
            match result {
                Ok(code) => code,
                Err(run_error) => {
                    // TODO: See about returning error code as well
                    run_error.error
                }
            }
        }
        Err(err) => err,
    }
}
