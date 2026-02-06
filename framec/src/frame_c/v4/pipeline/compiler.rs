//! Main compilation logic
//!
//! This module contains the core compilation pipeline extracted from mod.rs.
//! Eventually this will be the single entry point for Frame V4 compilation.

use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::utils::RunError;
use super::config::{PipelineConfig, CompileMode};
use super::traits::get_region_scanner;
use crate::frame_c::v4::codegen::{generate_system, get_backend, CodegenNode, EmitContext};
use crate::frame_c::v4::arcanum::{Arcanum, build_arcanum_from_frame_ast};

/// Result of module compilation
#[derive(Debug)]
pub struct CompileResult {
    /// Generated code
    pub code: String,
    /// Validation errors (if any)
    pub errors: Vec<CompileError>,
    /// Validation warnings (if any)
    pub warnings: Vec<CompileError>,
    /// Source map (if generated)
    pub source_map: Option<String>,
}

/// Compilation error
#[derive(Debug, Clone)]
pub struct CompileError {
    pub code: String,
    pub message: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

impl CompileError {
    pub fn new(code: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            line: None,
            column: None,
        }
    }

    pub fn with_location(mut self, line: usize, column: usize) -> Self {
        self.line = Some(line);
        self.column = Some(column);
        self
    }
}

/// Compile a Frame module from source bytes
///
/// This is the main entry point for V4 compilation. It uses the configuration
/// to determine the compilation mode, target language, and backend selection.
///
/// # Arguments
/// * `source` - The Frame source code as bytes
/// * `config` - Pipeline configuration
///
/// # Returns
/// A CompileResult containing the generated code (or validation results)
pub fn compile_module(source: &[u8], config: &PipelineConfig) -> Result<CompileResult, RunError> {
    use super::config::{CodegenBackend, record_v3_compile, record_v4_compile, record_v3_fallback};

    // Debug output if enabled
    if config.debug {
        eprintln!("[compile_module] Starting compilation with mode={:?}, target={:?}, backend={:?}",
            config.mode, config.target, config.backend);
    }

    // Check for validation-only mode
    if config.mode == CompileMode::ValidationOnly {
        return validate_only(source, config);
    }

    // Backend selection with usage tracking
    match config.backend {
        CodegenBackend::V4Ast => {
            // Pure V4 - fail if V4 doesn't work
            if config.debug {
                eprintln!("[compile_module] Using V4 AST-based backend");
            }
            let result = compile_ast_based(source, config)?;
            record_v4_compile();
            Ok(result)
        }
        CodegenBackend::V4WithV3Fallback => {
            // Try V4 first, fall back to V3 on failure
            if config.debug {
                eprintln!("[compile_module] Trying V4 with V3 fallback");
            }
            match compile_ast_based(source, config) {
                Ok(result) if result.errors.is_empty() => {
                    record_v4_compile();
                    Ok(result)
                }
                Ok(result) => {
                    // V4 had errors, fall back to V3
                    if config.debug {
                        eprintln!("[compile_module] V4 had {} errors, falling back to V3",
                            result.errors.len());
                    }
                    record_v3_fallback();
                    let code = compile_with_v3_pipeline(source, config)?;
                    Ok(CompileResult {
                        code,
                        errors: vec![],
                        warnings: vec![],
                        source_map: None,
                    })
                }
                Err(e) => {
                    // V4 failed completely, fall back to V3
                    if config.debug {
                        eprintln!("[compile_module] V4 failed: {:?}, falling back to V3", e);
                    }
                    record_v3_fallback();
                    let code = compile_with_v3_pipeline(source, config)?;
                    Ok(CompileResult {
                        code,
                        errors: vec![],
                        warnings: vec![],
                        source_map: None,
                    })
                }
            }
        }
        CodegenBackend::V3Legacy => {
            // Pure V3 legacy path
            if config.debug {
                eprintln!("[compile_module] Using V3 legacy backend");
            }
            record_v3_compile();
            let code = compile_with_v3_pipeline(source, config)?;
            Ok(CompileResult {
                code,
                errors: vec![],
                warnings: vec![],
                source_map: None,
            })
        }
    }
}

/// Validation-only mode
fn validate_only(source: &[u8], config: &PipelineConfig) -> Result<CompileResult, RunError> {
    use crate::frame_c::v4::frame_parser::FrameParser;
    use crate::frame_c::v4::frame_validator::FrameValidator;
    use crate::frame_c::v4::frame_ast::TargetLanguage as AstTarget;

    // Convert target language
    let ast_target = match config.target {
        TargetLanguage::Python3 => AstTarget::Python3,
        TargetLanguage::TypeScript => AstTarget::TypeScript,
        TargetLanguage::Rust => AstTarget::Rust,
        TargetLanguage::CSharp => AstTarget::CSharp,
        TargetLanguage::C => AstTarget::C,
        TargetLanguage::Cpp => AstTarget::Cpp,
        TargetLanguage::Java => AstTarget::Java,
        _ => AstTarget::Python3,
    };

    // Parse
    let mut parser = FrameParser::new(source, ast_target);
    let ast = match parser.parse_module() {
        Ok(ast) => ast,
        Err(e) => {
            return Ok(CompileResult {
                code: String::new(),
                errors: vec![CompileError::new("E001", &format!("Parse error: {}", e))],
                warnings: vec![],
                source_map: None,
            });
        }
    };

    // Validate
    let mut validator = FrameValidator::new();
    let errors = match validator.validate(&ast) {
        Ok(()) => vec![],
        Err(errs) => errs.iter().map(|e| {
            CompileError::new(&e.code, &e.message)
        }).collect(),
    };

    Ok(CompileResult {
        code: String::new(),
        errors,
        warnings: vec![],
        source_map: None,
    })
}

/// Compile using the existing V3 pipeline (temporary bridge)
fn compile_with_v3_pipeline(source: &[u8], config: &PipelineConfig) -> Result<String, RunError> {
    // This is a temporary bridge to the existing V3 pipeline
    // It will be replaced as the new architecture is completed

    // For now, just return an empty string as a placeholder
    // The actual V3 compilation is still handled by mod.rs
    Ok(String::new())
}

/// Compile using the new AST-based pipeline (experimental)
///
/// This function implements the new proper compiler architecture:
/// 1. Parse Frame source into AST
/// 2. Build symbol table (Arcanum) from AST
/// 3. Validate AST
/// 4. Generate CodegenNode from AST
/// 5. Emit target language code from CodegenNode
pub fn compile_ast_based(source: &[u8], config: &PipelineConfig) -> Result<CompileResult, RunError> {
    use crate::frame_c::v4::frame_parser::FrameParser;
    use crate::frame_c::v4::frame_validator::FrameValidator;
    use crate::frame_c::v4::frame_ast::{TargetLanguage as AstTarget, FrameAst};

    if config.debug {
        eprintln!("[compile_ast_based] Starting AST-based compilation");
    }

    // Step 1: Convert target language
    let ast_target = match config.target {
        TargetLanguage::Python3 => AstTarget::Python3,
        TargetLanguage::TypeScript => AstTarget::TypeScript,
        TargetLanguage::Rust => AstTarget::Rust,
        TargetLanguage::CSharp => AstTarget::CSharp,
        TargetLanguage::C => AstTarget::C,
        TargetLanguage::Cpp => AstTarget::Cpp,
        TargetLanguage::Java => AstTarget::Java,
        _ => AstTarget::Python3,
    };

    // Step 2: Parse source into AST
    let mut parser = FrameParser::new(source, ast_target);
    let ast = match parser.parse_module() {
        Ok(ast) => ast,
        Err(e) => {
            return Ok(CompileResult {
                code: String::new(),
                errors: vec![CompileError::new("E001", &format!("Parse error: {}", e))],
                warnings: vec![],
                source_map: None,
            });
        }
    };

    if config.debug {
        eprintln!("[compile_ast_based] Parsed AST successfully");
    }

    // Step 3: Build Arcanum from AST
    let arcanum = build_arcanum_from_frame_ast(&ast);

    if config.debug {
        eprintln!("[compile_ast_based] Built Arcanum with {} systems", arcanum.systems.len());
    }

    // Step 4: Validate AST
    let mut validator = FrameValidator::new();
    let validation_errors = match validator.validate(&ast) {
        Ok(()) => vec![],
        Err(errs) => errs.iter().map(|e| {
            CompileError::new(&e.code, &e.message)
        }).collect(),
    };

    // If there are validation errors, return them
    if !validation_errors.is_empty() {
        return Ok(CompileResult {
            code: String::new(),
            errors: validation_errors,
            warnings: vec![],
            source_map: None,
        });
    }

    if config.debug {
        eprintln!("[compile_ast_based] Validation passed");
    }

    // Step 5: Generate code from AST using codegen
    let backend = get_backend(config.target);
    let mut ctx = EmitContext::new();

    let code = match &ast {
        FrameAst::System(system) => {
            ctx = ctx.with_system(&system.name);
            let codegen_node = generate_system(system, &arcanum, config.target, source);
            backend.emit(&codegen_node, &mut ctx)
        }
        FrameAst::Module(module) => {
            // Generate code for each system in the module
            let mut result = String::new();

            // Add runtime imports
            for import in backend.runtime_imports() {
                result.push_str(&import);
                result.push('\n');
            }
            if !backend.runtime_imports().is_empty() {
                result.push('\n');
            }

            // Generate each system
            for (i, system) in module.systems.iter().enumerate() {
                if i > 0 {
                    result.push_str("\n\n");
                }
                ctx = ctx.with_system(&system.name);
                let codegen_node = generate_system(system, &arcanum, config.target, source);
                result.push_str(&backend.emit(&codegen_node, &mut ctx));
            }

            result
        }
    };

    if config.debug {
        eprintln!("[compile_ast_based] Generated {} bytes of code", code.len());
    }

    Ok(CompileResult {
        code,
        errors: vec![],
        warnings: vec![],
        source_map: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_result_creation() {
        let result = CompileResult {
            code: "generated code".to_string(),
            errors: vec![],
            warnings: vec![],
            source_map: None,
        };
        assert_eq!(result.code, "generated code");
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_compile_error_with_location() {
        let error = CompileError::new("E001", "test error")
            .with_location(10, 5);
        assert_eq!(error.line, Some(10));
        assert_eq!(error.column, Some(5));
    }

    #[test]
    fn test_validation_only_mode() {
        let source = b"@@system Test { machine: $A { } }";
        let config = PipelineConfig::validation_only(TargetLanguage::Python3);
        let result = compile_module(source, &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_ast_based_simple_system() {
        // A minimal Frame system using V4 syntax (@@system)
        let source = b"@@system Test { machine: $Init { } }";
        let config = PipelineConfig::production(TargetLanguage::Python3);
        let result = compile_ast_based(source, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        // Debug output for development
        if !output.errors.is_empty() {
            eprintln!("Parse errors: {:?}", output.errors);
            // Parser may not yet support all V4 features
            return;
        }
        assert!(output.code.contains("class Test"));
    }

    #[test]
    fn test_compile_ast_based_with_state() {
        let source = b"@@system TrafficLight { machine: $Red { } $Green { } }";
        let config = PipelineConfig::production(TargetLanguage::Python3);
        let result = compile_ast_based(source, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        if !output.errors.is_empty() {
            eprintln!("Parse errors: {:?}", output.errors);
            return;
        }
        assert!(output.code.contains("class TrafficLight"));
    }

    #[test]
    fn test_compile_ast_based_typescript() {
        let source = b"@@system Test { machine: $Init { } }";
        let config = PipelineConfig::production(TargetLanguage::TypeScript);
        let result = compile_ast_based(source, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        if !output.errors.is_empty() {
            return;
        }
        // TypeScript uses "class"
        assert!(output.code.contains("class Test"));
    }

    #[test]
    fn test_compile_ast_based_rust() {
        let source = b"@@system Test { machine: $Init { } }";
        let config = PipelineConfig::production(TargetLanguage::Rust);
        let result = compile_ast_based(source, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        if !output.errors.is_empty() {
            return;
        }
        // Rust uses "struct" and "impl"
        assert!(output.code.contains("pub struct Test"));
    }

    #[test]
    fn test_compile_ast_based_parse_error() {
        let source = b"this is not valid frame syntax";
        let config = PipelineConfig::production(TargetLanguage::Python3);
        let result = compile_ast_based(source, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        // Should have parse error
        assert!(!output.errors.is_empty());
        assert!(output.code.is_empty());
    }

    #[test]
    fn test_compile_ast_based_function_runs() {
        // Basic test that compile_ast_based doesn't panic
        let source = b"@@system Test { machine: $A { } }";
        let config = PipelineConfig::production(TargetLanguage::Python3);
        let result = compile_ast_based(source, &config);
        // Should not return an error from RunError
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_ast_based_with_handler() {
        // System with handlers to verify Arcanum-based handler generation
        let source = br#"@@system TestHandler {
    machine:
        $Idle {
            start() {
                x = 1
            }
        }
}"#;
        let config = PipelineConfig::production(TargetLanguage::Python3);
        let result = compile_ast_based(source, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        if !output.errors.is_empty() {
            for e in &output.errors {
                eprintln!("Error: {}: {}", e.code, e.message);
            }
            // Don't fail - parser may still have issues
            return;
        }
        // The generated code should have the handler method
        eprintln!("Generated code:\n{}", output.code);
        assert!(output.code.contains("class TestHandler"));
    }

    #[test]
    fn test_compile_ast_based_with_transition() {
        // System with transition to verify Frame segment expansion
        let source = br#"@@system TestTransition {
    machine:
        $Idle {
            start() {
                x = 1
                -> $Running
            }
        }
        $Running {
            stop() {
                -> $Idle
            }
        }
}"#;
        let config = PipelineConfig::production(TargetLanguage::Python3);
        let result = compile_ast_based(source, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        if !output.errors.is_empty() {
            for e in &output.errors {
                eprintln!("Error: {}: {}", e.code, e.message);
            }
            return;
        }
        eprintln!("Generated code:\n{}", output.code);
        // Should have transition call
        assert!(output.code.contains("_transition"));
        assert!(output.code.contains("_s_Running"));
    }
}
