pub mod frame_c;
use crate::compiler::Exe;
use crate::frame_c::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// Entry point for the online framepiler. See `frame_c::cli::run_with()` for
/// a more full-featured entry point.
pub fn run(frame_code: &str, format: &str) -> String {
    let exe = Exe::new();
    let result = exe.run(&None, frame_code.to_string(), format.to_string());
    match result {
        Ok(code) => code,
        Err(run_error) => {
            // TODO: See about returning error code as well
            run_error.error
        }
    }
}
