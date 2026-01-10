// Frame v4 Compiler - Adapter to use v3's proven components
// Only changes: 
// 1. Handle @@target pragma instead of v3 prolog
// 2. Everything else uses v3's battle-tested implementation

use crate::frame_c::v3::{
    native_region_scanner::NativeRegionScannerV3,
    mir_assembler::MirAssemblerV3,
    expander::FrameStatementExpanderV3,
    splice::SplicerV3,
    module_partitioner::ModulePartitionerV3,
    validator::BodyKindV3,
};
use super::{FrameV4Result, FrameV4Output, ErrorsAcc, TargetLanguage};

pub struct FrameV4CompilerAdapter {
    target: TargetLanguage,
}

impl FrameV4CompilerAdapter {
    pub fn new(target: TargetLanguage) -> Self {
        Self { target }
    }
    
    pub fn compile(&self, source: &str, file_name: &str) -> FrameV4Result {
        // Extract @@target pragma if present
        let _pragma_target = self.extract_target_pragma(source);
        
        // Convert @@target to @target for v3 compatibility
        let v3_source = source.replace("@@target ", "@target ");
        
        // Use v3's module partitioner to find system boundaries and bodies
        // Convert v4 target to v3 target
        let v3_target = match self.target {
            TargetLanguage::Python => crate::frame_c::visitors::TargetLanguage::Python3,
            TargetLanguage::TypeScript => crate::frame_c::visitors::TargetLanguage::TypeScript,
            TargetLanguage::Rust => crate::frame_c::visitors::TargetLanguage::Rust,
            TargetLanguage::C => crate::frame_c::visitors::TargetLanguage::C,
            TargetLanguage::Cpp => crate::frame_c::visitors::TargetLanguage::Cpp,
            TargetLanguage::Java => crate::frame_c::visitors::TargetLanguage::Java,
            TargetLanguage::CSharp => crate::frame_c::visitors::TargetLanguage::CSharp,
            _ => crate::frame_c::visitors::TargetLanguage::Python3,
        };
        
        let parts = match crate::frame_c::v3::module_partitioner::ModulePartitionerV3::partition(v3_source.as_bytes(), v3_target) {
            Ok(p) => p,
            Err(e) => {
                let mut errors = ErrorsAcc::new();
                errors.push_error(format!("Failed to partition module: {:?}", e));
                return FrameV4Result::Err(errors);
            }
        };
        
        // Process each body using v3's native scanner, MIR assembler, and expander
        let mut output = String::new();
        let mut cursor = 0usize;
        
        for body in &parts.bodies {
            // Copy content before this body
            if cursor < body.open_byte {
                output.push_str(&v3_source[cursor..body.open_byte]);
            }
            
            // Process the body
            let body_src = &v3_source[body.open_byte..=body.close_byte];
            let processed = match self.process_body(body_src, body.kind) {
                Ok(processed) => processed,
                Err(e) => {
                    let mut errors = ErrorsAcc::new();
                    errors.push_error(format!("Failed to process body: {}", e));
                    return FrameV4Result::Err(errors);
                }
            };
            
            output.push_str(&processed);
            cursor = body.close_byte + 1;
        }
        
        // Copy any remaining content
        if cursor < v3_source.len() {
            output.push_str(&v3_source[cursor..]);
        }
        
        // Convert back from @target to @@target in output
        let output = output.replace("@target python", "@@target python")
            .replace("@target python_3", "@@target python")
            .replace("@target typescript", "@@target typescript")
            .replace("@target rust", "@@target rust");
        
        FrameV4Result::Ok(FrameV4Output {
            code: output,
            warnings: Vec::new(),
            source_map: None,
        })
    }
    
    fn extract_target_pragma(&self, source: &str) -> Option<String> {
        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("@@target ") {
                let target = trimmed.trim_start_matches("@@target ").trim();
                return Some(target.to_string());
            }
            // Stop looking after first non-pragma line
            if !trimmed.is_empty() && !trimmed.starts_with("@@") && !trimmed.starts_with("//") {
                break;
            }
        }
        None
    }
    
    fn process_body(&self, body_src: &str, kind: BodyKindV3) -> Result<String, String> {
        use crate::frame_c::v3::native_region_scanner::{
            python::NativeRegionScannerPyV3,
            typescript::NativeRegionScannerTsV3,
            rust::NativeRegionScannerRustV3,
            RegionV3,
        };
        
        // Skip function bodies and unknowns
        if !matches!(kind, BodyKindV3::Handler | BodyKindV3::Action | BodyKindV3::Operation) {
            return Ok(body_src.to_string());
        }
        
        // Use v3's native scanner for the target language
        let scan = match self.target {
            TargetLanguage::Python => {
                let mut scanner = NativeRegionScannerPyV3;
                scanner.scan(body_src.as_bytes(), 0)
            }
            TargetLanguage::TypeScript => {
                let mut scanner = NativeRegionScannerTsV3;
                scanner.scan(body_src.as_bytes(), 0)
            }
            TargetLanguage::Rust => {
                let mut scanner = NativeRegionScannerRustV3;
                scanner.scan(body_src.as_bytes(), 0)
            }
            _ => {
                // Use Python for other languages
                let mut scanner = NativeRegionScannerPyV3;
                scanner.scan(body_src.as_bytes(), 0)
            }
        }.map_err(|e| format!("Scan error: {:?}", e))?;
        
        // Use v3's MIR assembler to parse Frame segments
        let mir = MirAssemblerV3.assemble(body_src.as_bytes(), &scan.regions)
            .map_err(|e| format!("MIR assembly error: {:?}", e))?;
        
        // Use custom v4 expander (not v3's which uses runtime libraries)
        let mut expansions = Vec::new();
        let mut mir_idx = 0;
        
        for region in &scan.regions {
            if let RegionV3::FrameSegment { indent, .. } = region {
                if mir_idx < mir.len() {
                    let expansion = self.expand_v4(&mir[mir_idx], *indent);
                    expansions.push(expansion);
                    mir_idx += 1;
                }
            }
        }
        
        // Use v3's splicer to replace Frame segments with expansions
        let spliced = SplicerV3.splice(body_src.as_bytes(), &scan.regions, &expansions);
        Ok(spliced.text)
    }
    
    fn expand_v4(&self, mir: &crate::frame_c::v3::mir::MirItemV3, indent: usize) -> String {
        use crate::frame_c::v3::mir::MirItemV3;
        
        let pad = " ".repeat(indent);
        
        match mir {
            MirItemV3::Transition { target, exit_args, enter_args, state_args, .. } => {
                match self.target {
                    TargetLanguage::Python => {
                        // Generate direct transition call without runtime libraries
                        let mut out = String::new();
                        out.push_str(&format!("{}self._transition_to_{}(", pad, target));
                        
                        // Add state arguments if present
                        if !state_args.is_empty() {
                            out.push_str(&state_args.join(", "));
                        }
                        
                        out.push_str(")\n");
                        out
                    }
                    TargetLanguage::TypeScript => {
                        // TypeScript transition
                        let mut out = String::new();
                        out.push_str(&format!("{}this._transition_to_{}(", pad, target));
                        if !state_args.is_empty() {
                            out.push_str(&state_args.join(", "));
                        }
                        out.push_str(");\n");
                        out
                    }
                    TargetLanguage::Rust => {
                        // Rust transition
                        let mut out = String::new();
                        out.push_str(&format!("{}self._transition_to_{}(", pad, target));
                        if !state_args.is_empty() {
                            out.push_str(&state_args.join(", "));
                        }
                        out.push_str(");\n");
                        out
                    }
                    _ => {
                        // Default to Python style
                        format!("{}self._transition_to_{}()\n", pad, target)
                    }
                }
            }
            MirItemV3::Forward { .. } => {
                match self.target {
                    TargetLanguage::Python => {
                        format!("{}# TODO: Forward to parent state\n", pad)
                    }
                    TargetLanguage::TypeScript => {
                        format!("{}// TODO: Forward to parent state\n", pad)
                    }
                    _ => {
                        format!("{}// TODO: Forward\n", pad)
                    }
                }
            }
            MirItemV3::StackPush { .. } => {
                match self.target {
                    TargetLanguage::Python => {
                        format!("{}self._state_stack.append(self._state)\n", pad)
                    }
                    TargetLanguage::TypeScript => {
                        format!("{}this._state_stack.push(this._state);\n", pad)
                    }
                    _ => {
                        format!("{}// TODO: Stack push\n", pad)
                    }
                }
            }
            MirItemV3::StackPop { .. } => {
                match self.target {
                    TargetLanguage::Python => {
                        let mut out = String::new();
                        out.push_str(&format!("{}if self._state_stack:\n", pad));
                        out.push_str(&format!("{}    prev_state = self._state_stack.pop()\n", pad));
                        out.push_str(&format!("{}    self._state = prev_state\n", pad));
                        out
                    }
                    TargetLanguage::TypeScript => {
                        let mut out = String::new();
                        out.push_str(&format!("{}if (this._state_stack.length > 0) {{\n", pad));
                        out.push_str(&format!("{}    const prev_state = this._state_stack.pop();\n", pad));
                        out.push_str(&format!("{}    this._state = prev_state;\n", pad));
                        out.push_str(&format!("{}}}\n", pad));
                        out
                    }
                    _ => {
                        format!("{}// TODO: Stack pop\n", pad)
                    }
                }
            }
        }
    }
}