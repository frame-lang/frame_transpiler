// Frame v4 Compiler that directly uses v3's complete compile_module pipeline
// This gives us the full v3 functionality while we build v4

use super::{FrameV4Result, FrameV4Output, ErrorsAcc, TargetLanguage};
use crate::frame_c::v3;
use crate::frame_c::visitors::TargetLanguage as V3TargetLanguage;

pub struct V3DirectCompiler {
    target: TargetLanguage,
}

impl V3DirectCompiler {
    pub fn new(target: TargetLanguage) -> Self {
        Self { target }
    }
    
    pub fn compile(&self, source: &str, _file_name: &str) -> FrameV4Result {
        // Convert @@target to @target for v3 compatibility
        let v3_source = source.replace("@@target ", "@target ");
        
        // Convert v4 target to v3 target
        let v3_target = match self.target {
            TargetLanguage::Python => V3TargetLanguage::Python3,
            TargetLanguage::TypeScript => V3TargetLanguage::TypeScript,
            TargetLanguage::Rust => V3TargetLanguage::Rust,
            TargetLanguage::C => V3TargetLanguage::C,
            TargetLanguage::Cpp => V3TargetLanguage::Cpp,
            TargetLanguage::Java => V3TargetLanguage::Java,
            TargetLanguage::CSharp => V3TargetLanguage::CSharp,
            _ => V3TargetLanguage::Python3,
        };
        
        // Use v3's complete compile_module pipeline
        match v3::compile_module(&v3_source, v3_target) {
            Ok(code) => {
                FrameV4Result::Ok(FrameV4Output {
                    code,
                    warnings: Vec::new(),
                    source_map: None,
                })
            }
            Err(e) => {
                let mut errors = ErrorsAcc::new();
                errors.push_error(format!("V3 compilation error: {}", e));
                FrameV4Result::Err(errors)
            }
        }
    }
}