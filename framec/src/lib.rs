pub mod frame_c;
mod utils;
use crate::frame_c::*;
use crate::compiler::Exe;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(frame_code: &str, format:&str) -> String {
    let exe = Exe::new();
    exe.run(frame_code.to_string(),format.to_string())
}
