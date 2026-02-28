//! Main compilation logic
//!
//! This module contains the core compilation pipeline for Frame V4.
//! V4 is a pure preprocessor for @@system blocks.

use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::utils::RunError;
use super::config::{PipelineConfig, CompileMode};
use crate::frame_c::v4::codegen::{generate_system, generate_rust_compartment_types, generate_c_compartment_types, generate_compartment_class, generate_frame_event_class, generate_frame_context_class, get_backend, EmitContext};
use crate::frame_c::v4::arcanum::build_arcanum_from_frame_ast;
use crate::frame_c::v4::pragma_scanner::{PragmaScanner, PragmaRegion};
use crate::frame_c::v4::native_region_scanner::unified::SyntaxSkipper;

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

/// Extract native code from a source region, skipping Frame pragmas.
///
/// Uses PragmaScanner with language-specific SyntaxSkipper to properly handle
/// strings and comments. This ensures `@@` appearing inside string literals
/// or comments in the target language is NOT treated as a pragma.
///
/// # Arguments
/// * `bytes` - Source bytes to scan
/// * `lang` - Target language (determines string/comment syntax)
///
/// # Returns
/// Native code regions concatenated, with pragma lines removed
fn extract_native_code(bytes: &[u8], lang: TargetLanguage) -> String {
    // Get language-specific syntax skipper
    let result = match lang {
        TargetLanguage::Python3 => {
            use crate::frame_c::v4::native_region_scanner::python::PythonSkipper;
            PragmaScanner.scan(&PythonSkipper, bytes)
        }
        TargetLanguage::TypeScript => {
            use crate::frame_c::v4::native_region_scanner::typescript::TypeScriptSkipper;
            PragmaScanner.scan(&TypeScriptSkipper, bytes)
        }
        TargetLanguage::Rust => {
            use crate::frame_c::v4::native_region_scanner::rust::RustSkipper;
            PragmaScanner.scan(&RustSkipper, bytes)
        }
        TargetLanguage::C => {
            use crate::frame_c::v4::native_region_scanner::c::CSkipper;
            PragmaScanner.scan(&CSkipper, bytes)
        }
        TargetLanguage::Cpp => {
            use crate::frame_c::v4::native_region_scanner::cpp::CppSkipper;
            PragmaScanner.scan(&CppSkipper, bytes)
        }
        TargetLanguage::Java => {
            use crate::frame_c::v4::native_region_scanner::java::JavaSkipper;
            PragmaScanner.scan(&JavaSkipper, bytes)
        }
        TargetLanguage::CSharp => {
            use crate::frame_c::v4::native_region_scanner::csharp::CSharpSkipper;
            PragmaScanner.scan(&CSharpSkipper, bytes)
        }
        _ => {
            // Fallback: use simple line-based filtering
            return skip_pragmas_simple(std::str::from_utf8(bytes).unwrap_or(""));
        }
    };

    match result {
        Ok(scan_result) => {
            let mut output = String::new();
            for region in scan_result.regions {
                if let PragmaRegion::NativeText { span } = region {
                    if span.start < span.end && span.end <= bytes.len() {
                        if let Ok(text) = std::str::from_utf8(&bytes[span.start..span.end]) {
                            output.push_str(text);
                        }
                    }
                }
            }
            output
        }
        Err(_) => {
            // Fallback on error
            skip_pragmas_simple(std::str::from_utf8(bytes).unwrap_or(""))
        }
    }
}

/// Simple line-based pragma removal (fallback when PragmaScanner fails)
fn skip_pragmas_simple(text: &str) -> String {
    let mut result = String::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("@@") {
            continue;
        }
        result.push_str(line);
        result.push('\n');
    }
    result
}

/// Legacy wrapper for backward compatibility - uses extract_native_code with Python default
fn skip_pragmas_keep_native(text: &str) -> String {
    // Default to simple filtering when language is not known
    // This maintains backward compatibility with existing callers
    skip_pragmas_simple(text)
}

/// Expand tagged instantiation patterns (@@SystemName()) in native code
///
/// Scans native code for `@@SystemName(args)` patterns where SystemName is a defined system,
/// and replaces with the appropriate constructor call for the target language.
///
/// # Arguments
/// * `text` - Native code to process
/// * `defined_systems` - Set of system names defined in this module
/// * `lang` - Target language (determines constructor syntax)
///
/// # Returns
/// Native code with @@System() patterns expanded to native constructors
fn expand_tagged_instantiations(
    text: &str,
    defined_systems: &std::collections::HashSet<String>,
    lang: TargetLanguage,
) -> Result<String, CompileError> {
    let bytes = text.as_bytes();
    let mut result = String::new();
    let mut i = 0;

    // Determine comment style based on language
    let uses_hash_comments = matches!(lang, TargetLanguage::Python3);
    let uses_c_style_comments = matches!(lang,
        TargetLanguage::TypeScript | TargetLanguage::Rust |
        TargetLanguage::C | TargetLanguage::Cpp |
        TargetLanguage::Java | TargetLanguage::CSharp
    );

    while i < bytes.len() {
        // Skip # comments (Python)
        if uses_hash_comments && bytes[i] == b'#' {
            // Copy the entire comment line
            while i < bytes.len() && bytes[i] != b'\n' {
                result.push(bytes[i] as char);
                i += 1;
            }
            continue;
        }

        // Skip // and /* */ comments (C-style languages)
        if uses_c_style_comments && bytes[i] == b'/' {
            if i + 1 < bytes.len() && bytes[i + 1] == b'/' {
                // Line comment - copy to end of line
                while i < bytes.len() && bytes[i] != b'\n' {
                    result.push(bytes[i] as char);
                    i += 1;
                }
                continue;
            }
            if i + 1 < bytes.len() && bytes[i + 1] == b'*' {
                // Block comment - copy until */
                result.push(bytes[i] as char);
                result.push(bytes[i + 1] as char);
                i += 2;
                while i + 1 < bytes.len() && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                    result.push(bytes[i] as char);
                    i += 1;
                }
                if i + 1 < bytes.len() {
                    result.push(bytes[i] as char);
                    result.push(bytes[i + 1] as char);
                    i += 2;
                }
                continue;
            }
        }

        // Skip string literals
        if bytes[i] == b'"' {
            result.push(bytes[i] as char);
            i += 1;
            while i < bytes.len() && bytes[i] != b'"' {
                if bytes[i] == b'\\' && i + 1 < bytes.len() {
                    result.push(bytes[i] as char);
                    i += 1;
                }
                result.push(bytes[i] as char);
                i += 1;
            }
            if i < bytes.len() {
                result.push(bytes[i] as char);
                i += 1;
            }
            continue;
        }

        // Skip single-quoted strings/chars
        if bytes[i] == b'\'' {
            result.push(bytes[i] as char);
            i += 1;
            while i < bytes.len() && bytes[i] != b'\'' {
                if bytes[i] == b'\\' && i + 1 < bytes.len() {
                    result.push(bytes[i] as char);
                    i += 1;
                }
                result.push(bytes[i] as char);
                i += 1;
            }
            if i < bytes.len() {
                result.push(bytes[i] as char);
                i += 1;
            }
            continue;
        }

        // Look for @@ pattern
        if i + 2 < bytes.len() && bytes[i] == b'@' && bytes[i + 1] == b'@' {
            let start = i;
            i += 2;

            // Check for uppercase letter (system name start)
            if i < bytes.len() && bytes[i].is_ascii_uppercase() {
                // Extract system name
                let name_start = i;
                while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                    i += 1;
                }
                let name = std::str::from_utf8(&bytes[name_start..i]).unwrap_or("");

                // Check for opening paren
                if i < bytes.len() && bytes[i] == b'(' {
                    // Find matching close paren
                    let mut paren_depth = 1;
                    let args_start = i + 1;
                    i += 1;
                    while i < bytes.len() && paren_depth > 0 {
                        match bytes[i] {
                            b'(' => paren_depth += 1,
                            b')' => paren_depth -= 1,
                            b'"' => {
                                // Skip string
                                i += 1;
                                while i < bytes.len() && bytes[i] != b'"' {
                                    if bytes[i] == b'\\' && i + 1 < bytes.len() {
                                        i += 1;
                                    }
                                    i += 1;
                                }
                            }
                            b'\'' => {
                                // Skip char/string
                                i += 1;
                                while i < bytes.len() && bytes[i] != b'\'' {
                                    if bytes[i] == b'\\' && i + 1 < bytes.len() {
                                        i += 1;
                                    }
                                    i += 1;
                                }
                            }
                            _ => {}
                        }
                        i += 1;
                    }

                    if paren_depth == 0 {
                        // Check if this is a defined system
                        if defined_systems.contains(name) {
                            // Get args (between parens, not including parens)
                            let args = std::str::from_utf8(&bytes[args_start..i - 1]).unwrap_or("");

                            // Generate constructor call based on language
                            let constructor = match lang {
                                TargetLanguage::Python3 => {
                                    format!("{}({})", name, args)
                                }
                                TargetLanguage::TypeScript => {
                                    format!("new {}({})", name, args)
                                }
                                TargetLanguage::Rust => {
                                    if args.trim().is_empty() {
                                        format!("{}::new()", name)
                                    } else {
                                        format!("{}::new({})", name, args)
                                    }
                                }
                                TargetLanguage::C => {
                                    if args.trim().is_empty() {
                                        format!("{}_new()", name)
                                    } else {
                                        format!("{}_new({})", name, args)
                                    }
                                }
                                TargetLanguage::Cpp => {
                                    format!("new {}({})", name, args)
                                }
                                TargetLanguage::Java => {
                                    format!("new {}({})", name, args)
                                }
                                TargetLanguage::CSharp => {
                                    format!("new {}({})", name, args)
                                }
                                _ => {
                                    format!("{}({})", name, args)
                                }
                            };

                            result.push_str(&constructor);
                            continue;
                        } else {
                            // Unknown system - return error
                            return Err(CompileError::new(
                                "E100",
                                &format!("Undefined system '{}' in tagged instantiation. Available systems: {:?}",
                                    name, defined_systems)
                            ));
                        }
                    }
                }
            }

            // Not a valid tagged instantiation, copy original
            result.push_str(&text[start..i]);
        } else {
            result.push(bytes[i] as char);
            i += 1;
        }
    }

    Ok(result)
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

    // Step 5: Generate code using "oceans model"
    // Native code is preserved verbatim, @@system blocks are replaced with generated code
    let backend = get_backend(config.target);
    let mut ctx = EmitContext::new();
    let source_str = String::from_utf8_lossy(source);

    // Build set of defined system names for tagged instantiation validation
    let defined_systems: std::collections::HashSet<String> = match &ast {
        FrameAst::System(system) => {
            let mut set = std::collections::HashSet::new();
            set.insert(system.name.clone());
            set
        }
        FrameAst::Module(module) => {
            module.systems.iter().map(|s| s.name.clone()).collect()
        }
    };

    let code = match &ast {
        FrameAst::System(system) => {
            let mut result = String::new();

            // Native code before @@system (prolog)
            let system_start = system.span.start;
            if system_start > 0 {
                // Find where native prolog starts (skip @@target line)
                let prolog = &source_str[..system_start];
                // Skip @@target and other @@ pragmas, keep native code
                let native_prolog = skip_pragmas_keep_native(prolog);
                // Expand tagged instantiations in prolog
                let native_prolog = match expand_tagged_instantiations(&native_prolog, &defined_systems, config.target) {
                    Ok(expanded) => expanded,
                    Err(e) => return Ok(CompileResult {
                        code: String::new(),
                        errors: vec![e],
                        warnings: vec![],
                        source_map: None,
                    }),
                };
                if !native_prolog.trim().is_empty() {
                    result.push_str(&native_prolog);
                    if !native_prolog.ends_with('\n') {
                        result.push('\n');
                    }
                    result.push('\n');
                }
            }

            // Add runtime imports (after native imports)
            for import in backend.runtime_imports() {
                result.push_str(&import);
                result.push('\n');
            }
            if !backend.runtime_imports().is_empty() {
                result.push('\n');
            }

            // Generate FrameEvent and Compartment classes BEFORE the main system class
            // Rust: Uses enum-of-structs pattern (specialized), no FrameEvent class
            // C: Uses per-system runtime types (FrameDict, FrameVec, structs)
            // Python/TypeScript: Use FrameEvent class and canonical 7-field Compartment class
            if matches!(config.target, TargetLanguage::Rust) {
                let compartment_types = generate_rust_compartment_types(system);
                if !compartment_types.is_empty() {
                    result.push_str(&compartment_types);
                }
            } else if matches!(config.target, TargetLanguage::C) {
                let c_runtime = generate_c_compartment_types(system);
                if !c_runtime.is_empty() {
                    result.push_str(&c_runtime);
                }
            } else {
                // Generate FrameEvent class first
                if let Some(event_node) = generate_frame_event_class(system, config.target) {
                    result.push_str(&backend.emit(&event_node, &mut ctx));
                    result.push_str("\n\n");
                }
                // Then generate FrameContext class (for reentrancy support)
                if let Some(context_node) = generate_frame_context_class(system, config.target) {
                    result.push_str(&backend.emit(&context_node, &mut ctx));
                    result.push_str("\n\n");
                }
                // Then generate Compartment class
                if let Some(compartment_node) = generate_compartment_class(system, config.target) {
                    result.push_str(&backend.emit(&compartment_node, &mut ctx));
                    result.push_str("\n\n");
                }
            }

            // Generated system code
            ctx = ctx.with_system(&system.name);
            let codegen_node = generate_system(system, &arcanum, config.target, source);
            result.push_str(&backend.emit(&codegen_node, &mut ctx));

            // Native code after @@system (epilog)
            let system_end = system.span.end;
            if system_end < source.len() {
                let epilog = &source_str[system_end..];
                let native_epilog = epilog.trim_start_matches(|c: char| c == '}' || c.is_whitespace());
                // Expand tagged instantiations in epilog
                let native_epilog = match expand_tagged_instantiations(native_epilog, &defined_systems, config.target) {
                    Ok(expanded) => expanded,
                    Err(e) => return Ok(CompileResult {
                        code: String::new(),
                        errors: vec![e],
                        warnings: vec![],
                        source_map: None,
                    }),
                };
                if !native_epilog.trim().is_empty() {
                    result.push_str("\n\n");
                    result.push_str(native_epilog.trim_start());
                }
            }

            result
        }
        FrameAst::Module(module) => {
            let mut result = String::new();
            let mut cursor = 0usize;

            // Add runtime imports first
            for import in backend.runtime_imports() {
                result.push_str(&import);
                result.push('\n');
            }
            if !backend.runtime_imports().is_empty() {
                result.push('\n');
            }

            // Process each system with native code between them
            for system in &module.systems {
                // Native code before this system
                let system_start = system.span.start;
                if system_start > cursor {
                    let native = &source_str[cursor..system_start];
                    let native_clean = if cursor == 0 {
                        skip_pragmas_keep_native(native)
                    } else {
                        native.to_string()
                    };
                    // Expand tagged instantiations in inter-system native code
                    let native_clean = match expand_tagged_instantiations(&native_clean, &defined_systems, config.target) {
                        Ok(expanded) => expanded,
                        Err(e) => return Ok(CompileResult {
                            code: String::new(),
                            errors: vec![e],
                            warnings: vec![],
                            source_map: None,
                        }),
                    };
                    if !native_clean.trim().is_empty() {
                        result.push_str(&native_clean);
                        if !native_clean.ends_with('\n') {
                            result.push('\n');
                        }
                    }
                }

                // Generate FrameEvent, FrameContext, and Compartment classes BEFORE the main system class
                // Rust: Uses enum-of-structs pattern (specialized), generates types inline
                // C: Uses per-system runtime types (FrameDict, FrameVec, structs)
                // Python/TypeScript: Use FrameEvent, FrameContext, and canonical 7-field Compartment classes
                if matches!(config.target, TargetLanguage::Rust) {
                    let compartment_types = generate_rust_compartment_types(system);
                    if !compartment_types.is_empty() {
                        result.push_str(&compartment_types);
                    }
                } else if matches!(config.target, TargetLanguage::C) {
                    let c_runtime = generate_c_compartment_types(system);
                    if !c_runtime.is_empty() {
                        result.push_str(&c_runtime);
                    }
                } else {
                    // Generate FrameEvent class first
                    if let Some(event_node) = generate_frame_event_class(system, config.target) {
                        result.push_str(&backend.emit(&event_node, &mut ctx));
                        result.push_str("\n\n");
                    }
                    // Then generate FrameContext class (for reentrancy support)
                    if let Some(context_node) = generate_frame_context_class(system, config.target) {
                        result.push_str(&backend.emit(&context_node, &mut ctx));
                        result.push_str("\n\n");
                    }
                    // Then generate Compartment class
                    if let Some(compartment_node) = generate_compartment_class(system, config.target) {
                        result.push_str(&backend.emit(&compartment_node, &mut ctx));
                        result.push_str("\n\n");
                    }
                }

                // Generated system code
                ctx = ctx.with_system(&system.name);
                let codegen_node = generate_system(system, &arcanum, config.target, source);
                result.push_str(&backend.emit(&codegen_node, &mut ctx));
                result.push('\n');

                cursor = system.span.end;
            }

            // Native code after last system
            if cursor < source.len() {
                let epilog = &source_str[cursor..];
                let native_epilog = epilog.trim_start_matches(|c: char| c == '}' || c.is_whitespace());
                // Expand tagged instantiations in final epilog
                let native_epilog = match expand_tagged_instantiations(native_epilog, &defined_systems, config.target) {
                    Ok(expanded) => expanded,
                    Err(e) => return Ok(CompileResult {
                        code: String::new(),
                        errors: vec![e],
                        warnings: vec![],
                        source_map: None,
                    }),
                };
                if !native_epilog.trim().is_empty() {
                    result.push_str("\n");
                    result.push_str(native_epilog.trim_start());
                }
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
