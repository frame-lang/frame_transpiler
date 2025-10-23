use crate::frame_c::config::FrameConfig;
use crate::frame_c::parser::*;
use crate::frame_c::scanner::*;
use crate::frame_c::source_map::SourceMapBuilder;
use crate::frame_c::symbol_table::*;
use crate::frame_c::utils::{frame_exitcode, RunError};
use crate::frame_c::visitors::python_visitor::PythonVisitor;
use crate::frame_c::visitors::graphviz_visitor::GraphVizVisitor;
use crate::frame_c::modules::MultiFileCompiler;
use crate::frame_c::ast_serialize::{serialize_ast_to_json, save_ast_to_file, ast_summary, generate_line_map};
use crate::frame_c::ast::NodeElement;
use std::cell::RefCell;
use std::rc::Rc;

use exitcode::USAGE;
use sha2::{Digest, Sha256};
use std::fs;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};

// Re-export this enum here since it's part of the interface for the run functions. The definition
// lives with visitors since adding a new visitor requires extending the enum and its trait impls.
use crate::frame_c::ast::AttributeNode;
pub use crate::frame_c::visitors::TargetLanguage;
use std::convert::TryFrom;

/* --------------------------------------------------------------------- */

static IS_DEBUG: bool = false;
static FRAMEC_VERSION: &str = concat!("Emitted from framec_v", env!("FRAME_VERSION"));

/* --------------------------------------------------------------------- */

pub struct Exe {}

impl Exe {
    /* --------------------------------------------------------------------- */

    pub fn new() -> Exe {
        Exe {}
    }

    pub fn debug_print(msg: &str) {
        if !IS_DEBUG {
            return;
        }

        eprintln!("{}", msg);
    }

    /* --------------------------------------------------------------------- */

    /// Run the Frame compiler on a Frame specification loaded from a file.
    ///
    /// # Arguments
    ///
    /// * `input_path` - Path to the file containing the Frame specification.
    ///
    /// * `target_language` - The target language to compile the specification to. This may be
    ///   `None` if the `language` attribute is defined in the specification itself.
    pub fn run_file(
        &self,
        input_path: &Path,
        target_language: Option<TargetLanguage>,
    ) -> Result<String, RunError> {
        match fs::read_to_string(input_path) {
            Ok(content) => {
                Exe::debug_print(&content);
                self.run(input_path.to_str(), content, target_language)
            }
            Err(err) => {
                let error_msg = format!("Cannot read file: {}", err);
                let run_error = RunError::new(exitcode::NOINPUT, &*error_msg);
                Err(run_error)
            }
        }
    }
    
    /// Run the Frame compiler with debug output (JSON with code and source map)
    ///
    /// # Arguments
    ///
    /// * `input_path` - Path to the file containing the Frame specification.
    ///
    /// * `target_language` - The target language to compile the specification to.
    pub fn run_file_debug(
        &self,
        input_path: &Path,
        target_language: Option<TargetLanguage>,
    ) -> Result<String, RunError> {
        match fs::read_to_string(input_path) {
            Ok(content) => {
                Exe::debug_print(&content);
                self.run_debug(input_path, content, target_language)
            }
            Err(err) => {
                let error_msg = format!("Cannot read file: {}", err);
                let run_error = RunError::new(exitcode::NOINPUT, &*error_msg);
                Err(run_error)
            }
        }
    }

    /* --------------------------------------------------------------------- */
    
    /// Run the Frame compiler in multi-file mode on a Frame project.
    ///
    /// # Arguments
    ///
    /// * `entry_path` - Path to the entry point file of the Frame project.
    ///
    /// * `target_language` - The target language to compile to.
    pub fn run_multifile(
        &self,
        entry_path: &Path,
        target_language: Option<TargetLanguage>,
        output_dir: Option<PathBuf>,
    ) -> Result<String, RunError> {
        // Support Python and TypeScript targets for multi-file
        let lang = target_language.unwrap_or(TargetLanguage::Python3);
        if !matches!(lang, TargetLanguage::Python3 | TargetLanguage::TypeScript) {
            let error_msg = "Multi-file compilation is only supported for Python and TypeScript target languages";
            return Err(RunError::new(exitcode::USAGE, error_msg));
        }
        
        // Create default config
        let config = FrameConfig::default();
        
        // Create and run multi-file compiler
        let mut compiler = MultiFileCompiler::new_for_entry(config, entry_path, lang).map_err(|e| {
            RunError::new(frame_exitcode::PARSE_ERR, &format!("Cannot initialize multi-file compiler: {}", e))
        })?;
        
        // If output_dir is specified, use separate files strategy
        if let Some(dir) = output_dir {
            compiler.set_output_dir(dir);
        }
        
        let output = compiler.compile(entry_path).map_err(|e| {
            RunError::new(frame_exitcode::PARSE_ERR, &format!("Multi-file compilation failed: {}", e))
        })?;
        
        Ok(output)
    }

    /* --------------------------------------------------------------------- */

    pub fn run_stdin(
        &self,
        target_language: Option<TargetLanguage>,
    ) -> Result<String, RunError> {
        let mut buffer = String::new();
        let mut stdin = io::stdin(); // We get `Stdin` here.
        match stdin.read_to_string(&mut buffer) {
            Ok(_size) => {
                Exe::debug_print(&buffer);
                self.run(None, buffer, target_language)
            }
            Err(err) => {
                let error_msg = format!("Cannot read file: {}", err);
                let run_error = RunError::new(exitcode::NOINPUT, &*error_msg);
                Err(run_error)
            }
        }
    }

    /* --------------------------------------------------------------------- */

    /// Run the Frame compiler on a Frame specification passed as a `String`.
    ///
    /// # Arguments
    ///
    /// * `input_path_str` - Path to the file containing the Frame specification, as a `&str`.
    ///   This value is used for metadata in some backends, but the file path is not verified or
    ///   loaded. This argument may be `None` if the Frame specification does not exist on the file
    ///   system, for example, if it was obtained from standard input or the online framepiler.
    ///
    /// * `content` - The Frame specification.
    ///
    /// * `target_language` - The target language to compile the specification to. This may be
    ///   `None` if the `language` attribute is defined in the specification itself.

    pub fn run(
        &self,
        _input_path_str: Option<&str>,
        content: String,
        mut target_language: Option<TargetLanguage>,
    ) -> Result<String, RunError> {
        // NOTE!!! There is a bug w/ the CLion debugger when a variable (maybe just String type)
        // isn't initialized under some circumstances. Basically the debugger
        // stops debugging or doesn't step and it looks like it hangs. To avoid
        // this you have to initialize the variable, but the compiler then complains
        // about the unused assignment. This can be squelched with `#[allow(unused_assignments)]`
        // but I've reported it to JetBrains and want it fixed. So when you are
        // debugging here, just uncomment the next line and then comment it back
        // when checking in.

        let mut hasher = Sha256::new();
        hasher.update(&content);
        // let sha256 = &format!("{:x}", hasher.finalize());

        let mut output;
        //        let mut output= String::new(); ^^^^ See above! ^^^^

        let scanner = Scanner::new(content);

        let (has_errors, errors, tokens) = scanner.scan_tokens();
        if has_errors {
            let run_error = RunError::new(frame_exitcode::PARSE_ERR, &*errors);
            return Err(run_error);
        }

        for token in &tokens {
            Exe::debug_print(&format!("{:?}", token));
        }

        let mut arcanum = Arcanum::new();
        let mut comments = Vec::new();
        // First pass: syntactic parsing to build symbol table
        {
            if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                eprintln!("DEBUG: Starting first pass - building symbol table");
            }
            let mut syntactic_parser = Parser::new(&tokens, &mut comments, true, arcanum);
            if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                eprintln!("DEBUG: Created syntactic parser with is_building_symbol_table=true");
            }

            // Check for parser errors before parsing
            if syntactic_parser.had_error() {
                let mut errors = "Initial parser errors:\n".to_string();
                errors.push_str(&syntactic_parser.get_errors());
                let run_error = RunError::new(frame_exitcode::PARSE_ERR, &errors);
                return Err(run_error);
            }

            match syntactic_parser.parse() {
                Ok(_) => {
                    if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                        eprintln!("DEBUG: First pass parsing succeeded");
                    }
                    // Check for errors after parsing but before consuming the parser
                    if syntactic_parser.had_error() {
                        let mut errors = String::new();
                        errors.push_str(&syntactic_parser.get_errors());
                        let run_error = RunError::new(frame_exitcode::PARSE_ERR, &errors);
                        return Err(run_error);
                    }
                    // Symbol table building successful, extract arcanum for second pass
                    arcanum = syntactic_parser.get_arcanum();
                    if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                        eprintln!("DEBUG: Extracted arcanum from first pass");
                    }
                    // Debug: Check what symbols are in the arcanum before second pass
                    let current_table = arcanum.current_symtab.borrow();
                    let symbol_keys: Vec<String> = current_table.symbols.keys().cloned().collect();
                    if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                        eprintln!("DEBUG: Symbols in arcanum after first pass: {:?}", symbol_keys);
                    }
                }
                Err(parse_error) => {
                    let mut errors = String::new();
                    
                    // Check if parser has accumulated detailed errors (includes line numbers)
                    if syntactic_parser.had_error() {
                        errors.push_str(&syntactic_parser.get_errors());
                    } else {
                        // Fallback to ParseError if no accumulated errors
                        errors.push_str(&parse_error.error);
                    }
                    
                    errors.push_str("\n\nPlease check your Frame syntax for missing braces, semicolons, or malformed blocks.");
                    let run_error = RunError::new(frame_exitcode::PARSE_ERR, &errors);
                    return Err(run_error);
                }
            }
        }

        let mut comments2 = comments.clone();
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG: Starting second pass - semantic analysis");
        }
        // Debug: Check if symbols are still there before creating second parser
        let module_table = arcanum.module_symtab.borrow();
        let module_symbol_keys: Vec<String> = module_table.symbols.keys().cloned().collect();
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG: Symbols in module scope before second pass: {:?}", module_symbol_keys);
        }
        drop(module_table);
        let mut semantic_parser = Parser::new(&tokens, &mut comments2, false, arcanum);
        
        
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG: Created semantic parser with is_building_symbol_table=false");
        }
        
        // Parse with proper error handling - no more fallback architecture
        let frame_module = match semantic_parser.parse() {
            Ok(module) => module,
            Err(parse_error) => {
                // Check if we have accumulated errors with line numbers first
                if semantic_parser.had_error() {
                    let mut errors = "Terminating with errors.\n".to_string();
                    errors.push_str(&semantic_parser.get_errors());
                    let run_error = RunError::new(frame_exitcode::PARSE_ERR, &errors);
                    return Err(run_error);
                } else {
                    // Fall back to ParseError message if no accumulated errors
                    let mut errors = "Parse error:\n".to_string();
                    errors.push_str(&parse_error.error);
                    errors.push_str("\n\nParsing failed. Please check your Frame syntax and try again.");
                    let run_error = RunError::new(frame_exitcode::PARSE_ERR, &errors);
                    return Err(run_error);
                }
            }
        };
        
        if semantic_parser.had_error() {
            let mut errors = "Terminating with errors.\n".to_string();
            errors.push_str(&semantic_parser.get_errors());
            let run_error = RunError::new(frame_exitcode::PARSE_ERR, &errors);
            return Err(run_error);
        }

        // AST Serialization for debugging (v0.60)
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG: AST Serialization enabled");
            
            // Generate AST summary
            let summary = ast_summary(&frame_module);
            eprintln!("DEBUG: {}", summary);
            
            // Generate line map
            let line_map = generate_line_map(&frame_module);
            eprintln!("DEBUG: Line Map:\n{}", line_map);
            
            // Full AST serialization
            match serialize_ast_to_json(&frame_module) {
                Ok(json) => {
                    eprintln!("DEBUG: Full AST JSON available (length: {} chars)", json.len());
                    
                    // Save to file if requested
                    if let Ok(output_file) = std::env::var("FRAME_AST_OUTPUT") {
                        match save_ast_to_file(&frame_module, &output_file) {
                            Ok(()) => eprintln!("DEBUG: AST saved to file: {}", output_file),
                            Err(e) => eprintln!("DEBUG: Failed to save AST to file: {}", e),
                        }
                    } else {
                        eprintln!("DEBUG: Set FRAME_AST_OUTPUT=filename.json to save AST to file");
                    }
                    
                    // Only print full JSON if explicitly requested (can be very large)
                    if std::env::var("FRAME_TRANSPILER_DEBUG_VERBOSE").is_ok() {
                        eprintln!("DEBUG: Full AST JSON:\n{}", json);
                    }
                }
                Err(e) => {
                    eprintln!("DEBUG: AST serialization error: {}", e);
                }
            }
        }

        // let generate_enter_args = semantic_parser.generate_enter_args;
        // let generate_exit_args = semantic_parser.generate_exit_args;
        let generate_state_context = semantic_parser.generate_state_context;
        let generate_state_stack = semantic_parser.generate_state_stack;
        let generate_change_state = semantic_parser.generate_change_state;
        let generate_transition_state = semantic_parser.generate_transition_state;

        // load configuration - always use defaults
        let config = FrameConfig::default();

        // check for language attribute override in spec specifying target language
        // v0.30: Check all systems for language attribute
        for system in &frame_module.systems {
            match &system.system_attributes_opt {
                Some(attributes) => {
                    if let Some(attr_node) = attributes.get("language") {
                        match attr_node {
                            AttributeNode::MetaNameValueStr { attr } => {
                                if let Ok(result) = TargetLanguage::try_from(attr.value.as_str()) {
                                    target_language = Some(result);
                                    break; // Use first language attribute found
                                }
                            }
                            _ => {}
                        }
                    }
                }
                None => {}
            }
        }

        match target_language {
            None => {
                let run_error = RunError::new(USAGE, "No target language specified.");
                return Err(run_error);
            }
            Some(lang) => match lang {
                TargetLanguage::Graphviz => {
                    let (arcanum, system_hierarchy_opt) = semantic_parser.get_all();
                    // If there was no system in the spec then don't run the visitor.
                    if let Some(system_hierarchy) = system_hierarchy_opt {
                        let mut visitor = GraphVizVisitor::new(
                            arcanum,
                            system_hierarchy,
                            generate_state_context,
                            generate_state_stack,
                            generate_change_state,
                            generate_transition_state,
                            FRAMEC_VERSION,
                            comments,
                        );
                        let results = visitor.run_v2(&frame_module);
                        
                        // For backward compatibility, concatenate all system DOT outputs
                        // with comments separating them
                        if results.is_empty() {
                            output = String::from(
                                "digraph structs { node [shape=plaintext] \
                                                        struct1 [label=\"No System\"]; \
                                                      }",
                            );
                        } else if results.len() == 1 {
                            // Single system - just output the DOT directly
                            output = results[0].1.clone();
                        } else {
                            // Multiple systems - concatenate with comments
                            output = format!("// Frame Module: {} systems\n", results.len());
                            for (system_name, dot_code) in results {
                                output.push_str(&format!("\n// System: {}\n", system_name));
                                output.push_str(&dot_code);
                                output.push_str("\n");
                            }
                        }
                    } else {
                        output = String::from(
                            "digraph structs { node [shape=plaintext] \
                                                    struct1 [label=\"No System\"]; \
                                                  }",
                        );
                    }
                }
                TargetLanguage::TypeScript => {
                    use crate::frame_c::visitors::typescript_visitor::TypeScriptVisitor;
                    use crate::frame_c::symbol_table::SymbolConfig;
                    
                    let arcanum = semantic_parser.get_arcanum();
                    let arcanum_vec = vec![arcanum];
                    let visitor = TypeScriptVisitor::new(
                        arcanum_vec,
                        SymbolConfig::new(),
                    );
                    output = visitor.run(&frame_module);
                }
                TargetLanguage::Python3 => {
                    // V2 is now the default, use USE_PYTHON_V1 to fallback to old visitor
                    if std::env::var("USE_PYTHON_V1").is_ok() {
                        // Use old visitor for backward compatibility testing
                        let mut visitor = PythonVisitor::new(
                            semantic_parser.get_arcanum(),
                            // generate_exit_args,
                            // generate_enter_args || generate_state_context,
                            generate_state_stack,
                            generate_change_state,
                            // generate_transition_state,
                            FRAMEC_VERSION,
                            comments,
                            config,
                        );
                        visitor.run_v2(&frame_module);
                        output = visitor.get_code();
                    } else {
                        // Use new V2 visitor with CodeBuilder architecture
                        use crate::frame_c::visitors::python_visitor_v2::PythonVisitorV2;
                        use crate::frame_c::symbol_table::SymbolConfig;
                        
                        let arcanum = semantic_parser.get_arcanum();
                        let arcanum_vec = vec![arcanum];
                        let mut visitor = PythonVisitorV2::new(
                            arcanum_vec,
                            SymbolConfig::new(),
                            config.clone(),
                            comments,
                        );
                        output = visitor.run(&frame_module);
                    }
                }
                TargetLanguage::Rust => {
                    // Use standard Rust visitor with working Frame semantics
                    use crate::frame_c::visitors::rust_visitor::RustVisitor;
                    use crate::frame_c::symbol_table::SymbolConfig;
                    
                    let arcanum = semantic_parser.get_arcanum();
                    let arcanum_vec = vec![arcanum];
                    let visitor = RustVisitor::new(
                        arcanum_vec,
                        SymbolConfig::new(),
                        config.clone(),
                        comments,
                    );
                    
                    output = visitor.run(&frame_module);
                }
                TargetLanguage::C => {
                    use crate::frame_c::visitors::c_visitor::CVisitor;
                    use crate::frame_c::symbol_table::SymbolConfig;
                    
                    let arcanum = semantic_parser.get_arcanum();
                    let arcanum_vec = vec![arcanum];
                    let visitor = CVisitor::new(
                        arcanum_vec,
                        SymbolConfig::new(),
                        config.clone(),
                        comments,
                    );
                    
                    output = visitor.run(&frame_module);
                }
            },
        }

        Ok(output)
    }
    
    /// Run the Frame compiler with source map tracking
    fn run_with_source_map(
        &self,
        _input_path_str: Option<&str>,
        content: String,
        mut target_language: Option<TargetLanguage>,
        source_map_builder: Rc<RefCell<SourceMapBuilder>>,
    ) -> Result<String, RunError> {
        // This is mostly the same as run(), but passes source_map_builder to PythonVisitor
        
        let mut hasher = Sha256::new();
        hasher.update(&content);

        let output;

        let scanner = Scanner::new(content);

        let (has_errors, errors, tokens) = scanner.scan_tokens();
        if has_errors {
            let run_error = RunError::new(frame_exitcode::PARSE_ERR, &*errors);
            return Err(run_error);
        }

        for token in &tokens {
            Exe::debug_print(&format!("{:?}", token));
        }

        let mut arcanum = Arcanum::new();
        let mut comments = Vec::new();
        // First pass: syntactic parsing to build symbol table
        {
            if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                eprintln!("DEBUG: Starting first pass - building symbol table");
            }
            let mut syntactic_parser = Parser::new(&tokens, &mut comments, true, arcanum);
            if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                eprintln!("DEBUG: Created syntactic parser with is_building_symbol_table=true");
            }

            // Check for parser errors before parsing
            if syntactic_parser.had_error() {
                let mut errors = "Initial parser errors:\n".to_string();
                errors.push_str(&syntactic_parser.get_errors());
                let run_error = RunError::new(frame_exitcode::PARSE_ERR, &errors);
                return Err(run_error);
            }

            match syntactic_parser.parse() {
                Ok(_) => {
                    if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                        eprintln!("DEBUG: First pass parsing succeeded");
                    }
                    // Check for errors after parsing but before consuming the parser
                    if syntactic_parser.had_error() {
                        let mut errors = String::new();
                        errors.push_str(&syntactic_parser.get_errors());
                        let run_error = RunError::new(frame_exitcode::PARSE_ERR, &errors);
                        return Err(run_error);
                    }
                    // Symbol table building successful, extract arcanum for second pass
                    arcanum = syntactic_parser.get_arcanum();
                    if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                        eprintln!("DEBUG: Extracted arcanum from first pass");
                    }
                    // Debug: Check what symbols are in the arcanum before second pass
                    let current_table = arcanum.current_symtab.borrow();
                    let symbol_keys: Vec<String> = current_table.symbols.keys().cloned().collect();
                    if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                        eprintln!("DEBUG: Symbols in arcanum after first pass: {:?}", symbol_keys);
                    }
                }
                Err(parse_error) => {
                    let mut errors = String::new();
                    
                    // Check if parser has accumulated detailed errors (includes line numbers)
                    if syntactic_parser.had_error() {
                        errors.push_str(&syntactic_parser.get_errors());
                    } else {
                        // Fallback to ParseError if no accumulated errors
                        errors.push_str(&parse_error.error);
                    }
                    
                    errors.push_str("\n\nPlease check your Frame syntax for missing braces, semicolons, or malformed blocks.");
                    let run_error = RunError::new(frame_exitcode::PARSE_ERR, &errors);
                    return Err(run_error);
                }
            }
        }

        let mut comments2 = comments.clone();
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG: Starting second pass - semantic analysis");
        }
        // Debug: Check if symbols are still there before creating second parser
        let module_table = arcanum.module_symtab.borrow();
        let module_symbol_keys: Vec<String> = module_table.symbols.keys().cloned().collect();
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG: Symbols in module scope before second pass: {:?}", module_symbol_keys);
        }
        drop(module_table);
        let mut semantic_parser = Parser::new(&tokens, &mut comments2, false, arcanum);
        
        
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG: Created semantic parser with is_building_symbol_table=false");
        }
        
        // Parse with proper error handling - no more fallback architecture
        let frame_module = match semantic_parser.parse() {
            Ok(module) => module,
            Err(parse_error) => {
                // Check if we have accumulated errors with line numbers first
                if semantic_parser.had_error() {
                    let mut errors = "Terminating with errors.\n".to_string();
                    errors.push_str(&semantic_parser.get_errors());
                    let run_error = RunError::new(frame_exitcode::PARSE_ERR, &errors);
                    return Err(run_error);
                } else {
                    // Fall back to ParseError message if no accumulated errors
                    let mut errors = "Parse error:\n".to_string();
                    errors.push_str(&parse_error.error);
                    errors.push_str("\n\nParsing failed. Please check your Frame syntax and try again.");
                    let run_error = RunError::new(frame_exitcode::PARSE_ERR, &errors);
                    return Err(run_error);
                }
            }
        };
        
        if semantic_parser.had_error() {
            let mut errors = "Terminating with errors.\n".to_string();
            errors.push_str(&semantic_parser.get_errors());
            let run_error = RunError::new(frame_exitcode::PARSE_ERR, &errors);
            return Err(run_error);
        }

        let _generate_state_context = semantic_parser.generate_state_context;
        let generate_state_stack = semantic_parser.generate_state_stack;
        let generate_change_state = semantic_parser.generate_change_state;
        let _generate_transition_state = semantic_parser.generate_transition_state;

        // load configuration - always use defaults
        let config = FrameConfig::default();

        // check for language attribute override in spec specifying target language
        // v0.30: Check all systems for language attribute
        for system in &frame_module.systems {
            match &system.system_attributes_opt {
                Some(attributes) => {
                    if let Some(attr_node) = attributes.get("language") {
                        match attr_node {
                            AttributeNode::MetaNameValueStr { attr } => {
                                if let Ok(result) = TargetLanguage::try_from(attr.value.as_str()) {
                                    target_language = Some(result);
                                    break; // Use first language attribute found
                                }
                            }
                            _ => {}
                        }
                    }
                }
                None => {}
            }
        }

        match target_language {
            None => {
                let run_error = RunError::new(USAGE, "No target language specified.");
                return Err(run_error);
            }
            Some(lang) => match lang {
                TargetLanguage::TypeScript => {
                    use crate::frame_c::visitors::typescript_visitor::TypeScriptVisitor;
                    use crate::frame_c::symbol_table::SymbolConfig;
                    
                    let arcanum = semantic_parser.get_arcanum();
                    let arcanum_vec = vec![arcanum];
                    let visitor = TypeScriptVisitor::new(
                        arcanum_vec,
                        SymbolConfig::new(),
                    );
                    output = visitor.run(&frame_module);
                }
                TargetLanguage::Python3 => {
                    // V2 is now the default with proper CodeBuilder source mapping
                    if std::env::var("USE_PYTHON_V1").is_ok() {
                        // Use old visitor if explicitly requested
                        let mut visitor = PythonVisitor::new(
                            semantic_parser.get_arcanum(),
                            generate_state_stack,
                            generate_change_state,
                            FRAMEC_VERSION,
                            comments,
                            config,
                        );
                        // Set the source map builder
                        visitor.set_source_map_builder(source_map_builder);
                        visitor.run_v2(&frame_module);
                        output = visitor.get_code();
                    } else {
                        // Use new V2 visitor with built-in CodeBuilder source mapping
                        use crate::frame_c::visitors::python_visitor_v2::PythonVisitorV2;
                        use crate::frame_c::symbol_table::SymbolConfig;
                        
                        let arcanum = semantic_parser.get_arcanum();
                        let arcanum_vec = vec![arcanum];
                        let mut visitor = PythonVisitorV2::new(
                            arcanum_vec,
                            SymbolConfig::new(),
                            config.clone(),
                            comments,
                        );
                        // Set the external source map builder for --debug-output integration
                        visitor.set_source_map_builder(source_map_builder);
                        output = visitor.run(&frame_module);
                        // V2 has integrated source mapping via CodeBuilder
                    }
                }
                TargetLanguage::Rust => {
                    // Use standard Rust visitor with working Frame semantics
                    use crate::frame_c::visitors::rust_visitor::RustVisitor;
                    use crate::frame_c::symbol_table::SymbolConfig;
                    
                    let arcanum = semantic_parser.get_arcanum();
                    let arcanum_vec = vec![arcanum];
                    let visitor = RustVisitor::new(
                        arcanum_vec,
                        SymbolConfig::new(),
                        config.clone(),
                        comments,
                    );
                    // Note: Standard Rust visitor doesn't use external source maps yet
                    
                    output = visitor.run(&frame_module);
                }
                TargetLanguage::C => {
                    use crate::frame_c::visitors::c_visitor::CVisitor;
                    use crate::frame_c::symbol_table::SymbolConfig;
                    
                    let arcanum = semantic_parser.get_arcanum();
                    let arcanum_vec = vec![arcanum];
                    let visitor = CVisitor::new(
                        arcanum_vec,
                        SymbolConfig::new(),
                        config.clone(),
                        comments,
                    );
                    // Note: C visitor doesn't use external source maps yet, but may in future
                    
                    output = visitor.run(&frame_module);
                }
                _ => {
                    let run_error = RunError::new(USAGE, "Source maps only supported for Python, Rust, and C targets.");
                    return Err(run_error);
                }
            },
        }

        Ok(output)
    }
    
    /// Run the Frame compiler with debug output, generating both code and source map
    pub fn run_debug(
        &self,
        input_path: &Path,
        content: String,
        mut target_language: Option<TargetLanguage>,
    ) -> Result<String, RunError> {
        use crate::frame_c::source_map::DebugOutput;
        
        // For now, only support Python and Rust for source maps
        if !matches!(target_language, Some(TargetLanguage::Python3) | Some(TargetLanguage::Rust) | None) {
            let error_msg = "Source map generation is only supported for Python and Rust target languages";
            return Err(RunError::new(exitcode::USAGE, error_msg));
        }
        
        // Set default to Python3 if not specified
        target_language = target_language.or(Some(TargetLanguage::Python3));
        
        // Create source map builder
        let source_file = input_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown.frm")
            .to_string();
        let target_file = match target_language {
            Some(TargetLanguage::Rust) => source_file.replace(".frm", ".rs"),
            _ => source_file.replace(".frm", ".py"), // Default to Python
        };
        let source_map_builder = Rc::new(RefCell::new(SourceMapBuilder::new(source_file, target_file)));
        
        // Run compilation with source map tracking
        let generated_code = self.run_with_source_map(
            input_path.to_str(),
            content.clone(),
            target_language,
            source_map_builder.clone()
        )?;
        
        // Build the final source map
        let source_map = source_map_builder.borrow().build();
        
        // Create debug output
        let debug_output = DebugOutput::new(generated_code, source_map, &content);
        
        // Serialize to JSON
        match serde_json::to_string_pretty(&debug_output) {
            Ok(json) => Ok(json),
            Err(e) => {
                let error_msg = format!("Cannot generate debug output: {}", e);
                Err(RunError::new(exitcode::SOFTWARE, &error_msg))
            }
        }
    }
}

impl Default for Exe {
    fn default() -> Self {
        Exe::new()
    }
}
