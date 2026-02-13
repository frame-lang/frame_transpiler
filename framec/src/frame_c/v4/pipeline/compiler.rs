//! Main compilation logic
//!
//! This module contains the core compilation pipeline for Frame V4.
//! V4 is a pure preprocessor for @@system blocks.

use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::utils::RunError;
use super::config::{PipelineConfig, CompileMode};
use crate::frame_c::v4::codegen::{generate_system, get_backend, EmitContext};
use crate::frame_c::v4::arcanum::build_arcanum_from_frame_ast;

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
/// This is the main entry point for V4 compilation.
///
/// # Arguments
/// * `source` - The Frame source code as bytes
/// * `config` - Pipeline configuration
///
/// # Returns
/// A CompileResult containing the generated code (or validation results)
pub fn compile_module(source: &[u8], config: &PipelineConfig) -> Result<CompileResult, RunError> {
    // Debug output if enabled
    if config.debug {
        eprintln!("[compile_module] Starting V4 compilation with mode={:?}, target={:?}",
            config.mode, config.target);
    }

    // Check for validation-only mode
    if config.mode == CompileMode::ValidationOnly {
        return validate_only(source, config);
    }

    // V4 AST-based compilation
    compile_ast_based(source, config)
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

/// Compile using the V4 AST-based pipeline
///
/// This function implements the V4 preprocessor architecture:
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

    // Step 4: Validate AST (using enhanced Arcanum-based validation)
    let mut validator = FrameValidator::new();
    let validation_errors = match validator.validate_with_arcanum(&ast, &arcanum) {
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
            let mut result = String::new();

            // Add runtime imports
            for import in backend.runtime_imports() {
                result.push_str(&import);
                result.push('\n');
            }
            if !backend.runtime_imports().is_empty() {
                result.push('\n');
            }

            ctx = ctx.with_system(&system.name);
            let codegen_node = generate_system(system, &arcanum, config.target, source);
            result.push_str(&backend.emit(&codegen_node, &mut ctx));
            result
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
    fn test_compile_simple_system() {
        let source = b"@@system Test { machine: $Init { } }";
        let config = PipelineConfig::production(TargetLanguage::Python3);
        let result = compile_module(source, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        if !output.errors.is_empty() {
            eprintln!("Parse errors: {:?}", output.errors);
            return;
        }
        assert!(output.code.contains("class Test"));
    }

    #[test]
    fn test_compile_with_transition() {
        let source = br#"@@system TestTransition {
    machine:
        $Idle {
            start() {
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
        let result = compile_module(source, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        if !output.errors.is_empty() {
            for e in &output.errors {
                eprintln!("Error: {}: {}", e.code, e.message);
            }
            return;
        }
        assert!(output.code.contains("_transition"));
    }

    #[test]
    fn test_compile_parse_error() {
        let source = b"this is not valid frame syntax";
        let config = PipelineConfig::production(TargetLanguage::Python3);
        let result = compile_module(source, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.errors.is_empty());
        assert!(output.code.is_empty());
    }
}
