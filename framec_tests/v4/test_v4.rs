// Test program for Frame v4 compiler

use framec::frame_c::v4::{FrameV4Compiler, TargetLanguage, FrameV4Result};
use std::fs;

fn main() {
    // Read test file
    let source = fs::read_to_string("framec_tests/v4/test_basic.fpy")
        .expect("Failed to read test file");
    
    // Create v4 compiler
    let compiler = FrameV4Compiler::new(TargetLanguage::Python);
    
    // Compile
    match compiler.compile(&source, "test_basic.fpy") {
        FrameV4Result::Ok(output) => {
            println!("✅ Compilation successful!");
            println!("\n--- Generated Code ---");
            println!("{}", output.code);
            
            // Write output
            fs::write("framec_tests/v4/test_basic.py", output.code)
                .expect("Failed to write output");
            
            if let Some(source_map) = output.source_map {
                println!("\n--- Source Map ---");
                println!("{}", source_map);
            }
        }
        FrameV4Result::Err(errors) => {
            println!("❌ Compilation failed!");
            println!("Errors: {:?}", errors);
        }
    }
}