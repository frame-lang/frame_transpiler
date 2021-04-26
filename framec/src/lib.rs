pub mod frame_c;
mod utils;
use crate::frame_c::*;
use crate::compiler::exe;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(frame_code: &str, format:&str) -> String {
    let exe = exe::new();
    exe.run(frame_code.to_string(),format.to_string())
}
