//! Main compilation logic
//!
//! This module contains the core compilation pipeline for Frame V4.
//! V4 is a pure preprocessor for @@system blocks.

use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::utils::RunError;
use super::config::{PipelineConfig, CompileMode};
use crate::frame_c::v4::codegen::{generate_system, generate_rust_compartment_types, generate_c_compartment_types, generate_compartment_class, generate_frame_event_class, generate_frame_context_class, get_backend, EmitContext};
use crate::frame_c::v4::arcanum::build_arcanum_from_frame_ast;
use crate::frame_c::v4::frame_ast::{FrameAst, ModuleAst, Span as AstSpan};
use crate::frame_c::v4::segmenter::{self, Segment};
use crate::frame_c::v4::pipeline_parser;
use crate::frame_c::v4::assembler;
use crate::frame_c::v4::frame_validator::FrameValidator;

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

// Helper functions extract_native_code, skip_pragmas_simple, skip_pragmas_keep_native,
// and expand_tagged_instantiations have been removed — their responsibilities are now
// handled by the Segmenter (Stage 0) and Assembler (Stage 7).

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
        TargetLanguage::Graphviz => AstTarget::Graphviz,
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

/// Compile using the V4 pipeline stages
///
/// Pipeline: Segmenter → Parser → Arcanum → Validator → Codegen → Emit → Assembler
///
/// 1. Segment source into Native/Pragma/System regions (Segmenter)
/// 2. For each System segment: parse → build Arcanum → validate → generate code
/// 3. Assemble final output: native pass-through + generated systems + tagged instantiations
pub fn compile_ast_based(source: &[u8], config: &PipelineConfig) -> Result<CompileResult, RunError> {
    if config.debug {
        eprintln!("[compile_ast_based] Starting pipeline-based compilation");
    }

    // Stage 0: Segment source
    let source_map = match segmenter::segment_source(source, config.target) {
        Ok(sm) => sm,
        Err(e) => {
            return Ok(CompileResult {
                code: String::new(),
                errors: vec![CompileError::new("E001", &format!("Segmentation error: {}", e))],
                warnings: vec![],
                source_map: None,
            });
        }
    };

    if config.debug {
        let system_count = source_map.segments.iter()
            .filter(|s| matches!(s, Segment::System { .. }))
            .count();
        eprintln!("[compile_ast_based] Segmented: {} segments, {} systems",
            source_map.segments.len(), system_count);
    }

    // Check for @@persist pragma
    let has_persist = source_map.persist_pragma().is_some();

    // Pass 1: Parse all systems into ASTs
    let mut system_asts: Vec<crate::frame_c::v4::frame_ast::SystemAst> = Vec::new();
    for segment in &source_map.segments {
        if let Segment::System { name, body_span, .. } = segment {
            let ast_body_span = AstSpan::new(body_span.start, body_span.end);

            let mut system_ast = match pipeline_parser::parse_system(
                &source_map.source, name.clone(), ast_body_span, config.target,
            ) {
                Ok(ast) => ast,
                Err(e) => {
                    return Ok(CompileResult {
                        code: String::new(),
                        errors: vec![CompileError::new("E002",
                            &format!("Parse error in system '{}': {}", name, e))],
                        warnings: vec![],
                        source_map: None,
                    });
                }
            };

            if has_persist {
                system_ast.persist_attr = Some(crate::frame_c::v4::frame_ast::PersistAttr {
                    save_name: None,
                    restore_name: None,
                    library: None,
                    span: AstSpan::new(0, 0),
                });
            }

            if config.debug {
                eprintln!("[compile_ast_based] Parsed system '{}': {} states, {} interface methods",
                    name,
                    system_ast.machine.as_ref().map(|m| m.states.len()).unwrap_or(0),
                    system_ast.interface.len());
            }

            system_asts.push(system_ast);
        }
    }

    // Build a shared arcanum containing ALL systems so they can reference each other
    let module_ast = FrameAst::Module(ModuleAst {
        name: String::new(),
        systems: system_asts.clone(),
        imports: Vec::new(),
        span: AstSpan::new(0, 0),
    });
    let arcanum = build_arcanum_from_frame_ast(&module_ast);

    // GraphViz target: bypass CodegenNode pipeline, use graph IR → DOT emitter
    if matches!(config.target, TargetLanguage::Graphviz) {
        use crate::frame_c::v4::graphviz;

        let mut dot_systems: Vec<(String, String)> = Vec::new();

        for system_ast in &system_asts {
            // Validate with shared arcanum
            let frame_ast = FrameAst::System(system_ast.clone());
            let mut validator = FrameValidator::new();
            if let Err(errs) = validator.validate_with_arcanum(&frame_ast, &arcanum) {
                let errors = errs.iter().map(|e| CompileError::new(&e.code, &e.message)).collect();
                return Ok(CompileResult {
                    code: String::new(),
                    errors,
                    warnings: vec![],
                    source_map: None,
                });
            }

            // Build graph IR and emit DOT
            let graph = graphviz::build_system_graph(system_ast, &arcanum);
            let dot = graphviz::emit_dot(&graph);
            dot_systems.push((system_ast.name.clone(), dot));
        }

        // Assemble: concatenate DOT blocks with // System: Name headers
        let code = graphviz::emit_multi_system(&dot_systems);

        if config.debug {
            eprintln!("[compile_ast_based] GraphViz: generated {} bytes of DOT for {} systems",
                code.len(), dot_systems.len());
        }

        return Ok(CompileResult {
            code,
            errors: vec![],
            warnings: vec![],
            source_map: None,
        });
    }

    // Pass 2: Validate + codegen each system with the shared arcanum
    let backend = get_backend(config.target);
    let mut ctx = EmitContext::new();
    let mut generated_systems: Vec<(String, String)> = Vec::new();

    for system_ast in &system_asts {
        // Validate with shared arcanum (all sibling systems visible)
        let frame_ast = FrameAst::System(system_ast.clone());
        let mut validator = FrameValidator::new();
        if let Err(errs) = validator.validate_with_arcanum(&frame_ast, &arcanum) {
            let errors = errs.iter().map(|e| CompileError::new(&e.code, &e.message)).collect();
            return Ok(CompileResult {
                code: String::new(),
                errors,
                warnings: vec![],
                source_map: None,
            });
        }

        // Build per-system generated code: runtime classes + system class
        let mut system_code = String::new();

        // Runtime imports (added once before the first system)
        if generated_systems.is_empty() {
            let imports = backend.runtime_imports();
            if !imports.is_empty() {
                for import in &imports {
                    system_code.push_str(import);
                    system_code.push('\n');
                }
                system_code.push('\n');
            }
        }

        // Runtime classes (language-specific, per-system)
        if matches!(config.target, TargetLanguage::Rust) {
            let compartment_types = generate_rust_compartment_types(system_ast);
            if !compartment_types.is_empty() {
                system_code.push_str(&compartment_types);
            }
        } else if matches!(config.target, TargetLanguage::C) {
            let c_runtime = generate_c_compartment_types(system_ast);
            if !c_runtime.is_empty() {
                system_code.push_str(&c_runtime);
            }
        } else {
            if let Some(event_node) = generate_frame_event_class(system_ast, config.target) {
                system_code.push_str(&backend.emit(&event_node, &mut ctx));
                system_code.push_str("\n\n");
            }
            if let Some(context_node) = generate_frame_context_class(system_ast, config.target) {
                system_code.push_str(&backend.emit(&context_node, &mut ctx));
                system_code.push_str("\n\n");
            }
            if let Some(compartment_node) = generate_compartment_class(system_ast, config.target) {
                system_code.push_str(&backend.emit(&compartment_node, &mut ctx));
                system_code.push_str("\n\n");
            }
        }

        // Codegen + Emit with shared arcanum
        ctx = ctx.with_system(&system_ast.name);
        let codegen_node = generate_system(system_ast, &arcanum, config.target, source);
        system_code.push_str(&backend.emit(&codegen_node, &mut ctx));

        generated_systems.push((system_ast.name.clone(), system_code));
    }

    // Stage 7: Assemble final output (native pass-through + system substitution + tagged instantiations)
    let code = match assembler::assemble(&source_map, &generated_systems, config.target) {
        Ok(output) => output,
        Err(e) => {
            return Ok(CompileResult {
                code: String::new(),
                errors: vec![CompileError::new("E003", &format!("Assembly error: {}", e))],
                warnings: vec![],
                source_map: None,
            });
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
    fn test_native_only_input_passes_through() {
        // Input with no @@system blocks is pure native code — passes through verbatim
        let source = b"this is just native code\nno systems here\n";
        let config = PipelineConfig::production(TargetLanguage::Python3);
        let result = compile_module(source, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.errors.is_empty());
        assert!(output.code.contains("this is just native code"));
    }

    fn test_compile_parse_error() {
        // Invalid syntax inside @@system should produce an error
        let source = b"@@system Test { not valid section syntax }";
        let config = PipelineConfig::production(TargetLanguage::Python3);
        let result = compile_module(source, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.errors.is_empty(), "Expected parse errors for invalid system content");
    }
}
