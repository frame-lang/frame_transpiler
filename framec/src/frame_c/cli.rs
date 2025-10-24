use crate::frame_c::compiler::{Exe, TargetLanguage};
use crate::frame_c::config::FrameConfig;
use clap::{Arg, Command};
use std::convert::TryFrom;
use std::path::PathBuf;

/// Command line arguments to the `framec` executable.
//#[derive(StructOpt)]
pub struct Cli {
    /// Stdin flag. Mutually exclusive with path
    stdin_flag: bool,

    /// Path to frame specification file.
    path: Option<PathBuf>,

    /// Target language.
    language: Option<String>,

    /// Multi-file project mode
    multifile: bool,

    /// Output directory for multi-file mode
    output_dir: Option<PathBuf>,

    /// Path to frame.toml config file
    config: Option<PathBuf>,

    /// Debug output mode - returns JSON with code and source map
    debug_output: bool,

    /// Enable syntax validation
    validate_syntax: bool,

    /// Validation level
    validation_level: Option<String>,

    /// Validation output format
    validation_format: Option<String>,

    /// Validation only mode (skip transpilation)
    validation_only: bool,

    /// Subcommand (build, init, etc.)
    command: Option<String>,
}

impl Cli {
    pub fn new() -> Cli {
        let matches = Command::new("framec")
            .version(env!("FRAME_VERSION"))
            .about("Frame language transpiler")
            .subcommand_required(false)
            .arg_required_else_help(false)
            .subcommand(Command::new("build").about("Build project using frame.toml configuration"))
            .subcommand(
                Command::new("init")
                    .about("Initialize a new Frame project with frame.toml")
                    .arg(
                        Arg::new("name")
                            .help("Project name")
                            .value_name("NAME")
                            .index(1),
                    ),
            )
            .arg(
                Arg::new("FILE-PATH")
                    .help("File path")
                    .value_name("FILE")
                    .index(1),
            )
            .arg(
                Arg::new("language")
                    .value_name("LANG")
                    .long("language")
                    .short('l')
                    .help("Target language (python_3, typescript, graphviz, rust, c)")
                    .long_help(
                        "Target language for code generation:\n  \
                               - python_3:       Python 3 with Frame runtime\n  \
                               - typescript:     TypeScript with state machine classes\n  \
                               - graphviz:       DOT format for state diagrams\n  \
                               - rust:           Type-safe Rust with generated visitor\n  \
                               - c:              C99 with Frame state machines",
                    )
                    .num_args(1),
            )
            .arg(
                Arg::new("multifile")
                    .long("multifile")
                    .short('m')
                    .help("Enable multi-file project compilation")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("output-dir")
                    .long("output-dir")
                    .short('o')
                    .help("Output directory for generated files (multi-file mode)")
                    .value_name("DIR")
                    .num_args(1),
            )
            .arg(
                Arg::new("config")
                    .long("config")
                    .short('c')
                    .help("Path to frame.toml configuration file")
                    .value_name("FILE")
                    .num_args(1),
            )
            .arg(
                Arg::new("debug-output")
                    .long("debug-output")
                    .help("Generate JSON output with transpiled code and source map for debugging")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("validate-syntax")
                    .long("validate-syntax")
                    .help("Enable comprehensive syntax and structural validation")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("validation-level")
                    .long("validation-level")
                    .help("Set validation level: basic, structural, semantic, target-language")
                    .value_name("LEVEL")
                    .num_args(1)
                    .value_parser(["basic", "structural", "semantic", "target-language"]),
            )
            .arg(
                Arg::new("validation-format")
                    .long("validation-format")
                    .help("Output format for validation results: human, json, junit")
                    .value_name("FORMAT")
                    .num_args(1)
                    .value_parser(["human", "json", "junit"]),
            )
            .arg(
                Arg::new("validation-only")
                    .long("validation-only")
                    .help("Run validation only, skip transpilation")
                    .action(clap::ArgAction::SetTrue),
            )
            .get_matches();

        // Check for subcommands first
        let command = match matches.subcommand() {
            Some((name, _)) => Some(name.to_string()),
            None => None,
        };

        let mut stdin = false;
        let mut path_opt = None;
        if !command.is_some() && matches.contains_id("FILE-PATH") {
            let file_path = matches.get_one::<String>("FILE-PATH");
            path_opt = file_path.map(|file_path| PathBuf::from(file_path.clone()));
        } else if !command.is_some() {
            stdin = true;
        }

        let language = matches.get_one::<String>("language");
        let language_opt = language.map(|lang| lang.clone());

        let multifile = matches.get_flag("multifile");

        let output_dir = matches.get_one::<String>("output-dir");
        let output_dir_opt = output_dir.map(|dir| PathBuf::from(dir.clone()));

        let config = matches.get_one::<String>("config");
        let config_opt = config.map(|cfg| PathBuf::from(cfg.clone()));

        let debug_output = matches.get_flag("debug-output");
        let validate_syntax = matches.get_flag("validate-syntax");
        let validation_level = matches
            .get_one::<String>("validation-level")
            .map(|s| s.clone());
        let validation_format = matches
            .get_one::<String>("validation-format")
            .map(|s| s.clone());
        let validation_only = matches.get_flag("validation-only");

        Cli {
            stdin_flag: stdin,
            path: path_opt,
            language: language_opt,
            multifile,
            output_dir: output_dir_opt,
            config: config_opt,
            debug_output,
            validate_syntax,
            validation_level,
            validation_format,
            validation_only,
            command,
        }
    }
}

impl Default for Cli {
    fn default() -> Self {
        Cli::new()
    }
}

/// Parse command-line arguments and run the compiler.
pub fn run() {
    run_with(Cli::new());
}

/// Run `framec` with the given CLI options.
pub fn run_with(args: Cli) {
    // Handle subcommands first
    if let Some(command) = args.command {
        match command.as_str() {
            "init" => {
                handle_init_command();
                return;
            }
            "build" => {
                handle_build_command(args.config);
                return;
            }
            _ => {
                eprintln!(
                    "Unknown command '{}'. Use 'framec --help' for available commands.",
                    command
                );
                std::process::exit(exitcode::USAGE);
            }
        }
    }

    // Original transpiler behavior
    let exe = Exe::new();

    let target_language = match &args.language {
        Some(lang_str) => match TargetLanguage::try_from(lang_str.clone()) {
            Ok(lang) => Some(lang),
            Err(err) => {
                eprintln!("Invalid target language: {}", err);
                std::process::exit(exitcode::USAGE);
            }
        },
        None => None,
    };

    // Handle validation if requested
    if args.validate_syntax || args.validation_only {
        let validation_result = handle_validation(&args, target_language);

        // If validation-only mode, exit after validation
        if args.validation_only {
            std::process::exit(if validation_result { 0 } else { 1 });
        }

        // If validation failed and we're not in validation-only mode,
        // still continue with transpilation but show the validation results
    }

    // run the compiler and print output to stdout
    if args.stdin_flag {
        match exe.run_stdin(target_language) {
            Ok(code) => {
                println!("{}", code);
            }
            Err(err) => {
                eprintln!("{}", err.error);
                std::process::exit(err.code);
            }
        }
    } else {
        let path = args.path.unwrap();
        let result = if args.debug_output {
            // Debug output mode - generate JSON with code and source map
            exe.run_file_debug(&path, target_language)
        } else if args.multifile {
            exe.run_multifile(&path, target_language, args.output_dir)
        } else {
            exe.run_file(&path, target_language)
        };

        match result {
            Ok(code) => {
                println!("{}", code);
            }
            Err(err) => {
                eprintln!("{}", err.error);
                std::process::exit(err.code);
            }
        }
    }
}

/// Handle the 'init' subcommand to create a new Frame project
fn handle_init_command() {
    use std::env;
    use std::fs;

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let config_path = current_dir.join("frame.toml");

    if config_path.exists() {
        eprintln!("frame.toml already exists in this directory");
        std::process::exit(exitcode::CANTCREAT);
    }

    // Get project name from directory name
    let project_name = current_dir
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string());

    match FrameConfig::create_default(&config_path, project_name.as_deref()) {
        Ok(_) => {
            println!("Created frame.toml");

            // Create src directory if it doesn't exist
            let src_dir = current_dir.join("src");
            if !src_dir.exists() {
                fs::create_dir(&src_dir).expect("Failed to create src directory");
                println!("Created src/");

                // Create a simple main.frm file
                let main_file = src_dir.join("main.frm");
                let main_content = r#"# Main entry point for Frame project

fn main() {
    print("Hello from Frame!")
}
"#;
                fs::write(&main_file, main_content).expect("Failed to create main.frm");
                println!("Created src/main.frm");
            }

            println!("\nFrame project initialized successfully!");
            println!("Run 'framec build' to compile your project.");
        }
        Err(e) => {
            eprintln!("Failed to create frame.toml: {}", e);
            std::process::exit(exitcode::IOERR);
        }
    }
}

/// Handle the 'build' subcommand using project configuration
fn handle_build_command(config_path: Option<PathBuf>) {
    // Load configuration
    let config = match FrameConfig::load(&config_path) {
        Ok(cfg) => cfg,
        Err(e) => {
            if config_path.is_some() {
                eprintln!("Failed to load config: {}", e);
            } else {
                eprintln!("No frame.toml found. Run 'framec init' to create one.");
            }
            std::process::exit(exitcode::CONFIG);
        }
    };

    // Use configuration to build
    let exe = Exe::new();
    let entry_point = config.entry_point();

    let target_language = match TargetLanguage::try_from(config.build.target.clone()) {
        Ok(lang) => Some(lang),
        Err(err) => {
            eprintln!("Invalid target language in config: {}", err);
            std::process::exit(exitcode::CONFIG);
        }
    };

    let output_dir = if config.use_separate_files() {
        Some(PathBuf::from(&config.build.output_dir))
    } else {
        None
    };

    // Always use multifile mode when building from config
    let result = exe.run_multifile(&entry_point, target_language, output_dir);

    match result {
        Ok(code) => {
            if config.use_separate_files() {
                println!(
                    "Build successful! Output written to {}/",
                    config.build.output_dir.display()
                );
            } else {
                println!("{}", code);
            }
        }
        Err(err) => {
            eprintln!("Build failed:\n{}", err.error);
            std::process::exit(err.code);
        }
    }
}

/// Handle validation logic
fn handle_validation(args: &Cli, target_language: Option<TargetLanguage>) -> bool {
    use crate::frame_c::parser::Parser;
    use crate::frame_c::scanner::Scanner;
    use crate::frame_c::symbol_table::Arcanum;
    use crate::frame_c::validation::*;
    use std::fs;

    // Ensure we have a file path for validation
    let path = match &args.path {
        Some(path) => path,
        None => {
            eprintln!("Error: File path required for validation");
            return false;
        }
    };

    // Read the source file
    let source_code = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file {}: {}", path.display(), err);
            return false;
        }
    };

    // Parse validation level
    let validation_level = match args.validation_level.as_deref() {
        Some("basic") => ValidationLevel::Basic,
        Some("structural") => ValidationLevel::Structural,
        Some("semantic") => ValidationLevel::Semantic,
        Some("target-language") => ValidationLevel::TargetLanguage,
        None => ValidationLevel::Structural, // Default
        Some(invalid) => {
            eprintln!("Error: Invalid validation level '{}'. Use: basic, structural, semantic, target-language", invalid);
            return false;
        }
    };

    // Parse output format
    let output_format = match args.validation_format.as_deref() {
        Some("human") => OutputFormat::Human,
        Some("json") => OutputFormat::Json,
        Some("junit") => OutputFormat::Junit,
        None => OutputFormat::Human, // Default
        Some(invalid) => {
            eprintln!(
                "Error: Invalid validation format '{}'. Use: human, json, junit",
                invalid
            );
            return false;
        }
    };

    // Convert target language
    use crate::frame_c::visitors::TargetLanguage as VisitorTargetLanguage;
    let target_lang = target_language.map(|tl| match tl {
        VisitorTargetLanguage::Python3 => crate::frame_c::validation::TargetLanguage::Python,
        VisitorTargetLanguage::TypeScript => crate::frame_c::validation::TargetLanguage::Python, // TODO: Add TypeScript validation
        VisitorTargetLanguage::Graphviz => crate::frame_c::validation::TargetLanguage::Python, // Default to Python for graphviz
        VisitorTargetLanguage::Rust => crate::frame_c::validation::TargetLanguage::Python, // TODO: Rust target not yet implemented
        VisitorTargetLanguage::C => crate::frame_c::validation::TargetLanguage::Python, // TODO: C target not yet implemented
    });

    // Create validation configuration
    let config = ValidationConfig {
        level: validation_level,
        target_language: target_lang,
        output_format,
        fail_on_warnings: false,
        max_errors: Some(100),
    };

    // Create validation engine with default rules and appropriate reporter
    let mut engine = ValidationEngine::with_default_rules(config);

    // Add the appropriate reporter based on output format
    match output_format {
        OutputFormat::Json => {
            engine =
                engine.add_reporter(crate::frame_c::validation::reporters::JsonReporter::new());
        }
        OutputFormat::Junit => {
            engine =
                engine.add_reporter(crate::frame_c::validation::reporters::JunitReporter::new());
        }
        OutputFormat::Human => {
            // Human reporter is already added by default
        }
        OutputFormat::Sarif => {
            // SARIF reporter not implemented yet - fall back to JSON
            engine =
                engine.add_reporter(crate::frame_c::validation::reporters::JsonReporter::new());
        }
    }

    // Parse the Frame file to get the actual AST using two-pass approach
    let scanner = Scanner::new(source_code.clone());
    let (has_errors, errors, tokens) = scanner.scan_tokens();

    if has_errors {
        eprintln!("Scanning errors: {}", errors);
        return false;
    }

    // First pass: symbol table building
    let mut arcanum = Arcanum::new();
    let mut comments = Vec::new();
    {
        let mut syntactic_parser = Parser::new(&tokens, &mut comments, true, arcanum);
        match syntactic_parser.parse() {
            Ok(_) => {
                if syntactic_parser.had_error() {
                    let mut errors = "First pass validation parsing errors:\n".to_string();
                    errors.push_str(&syntactic_parser.get_errors());
                    eprintln!("{}", errors);
                    return false;
                }
                arcanum = syntactic_parser.get_arcanum();
            }
            Err(parse_error) => {
                eprintln!("First pass validation parse error: {}", parse_error.error);
                return false;
            }
        }
    }

    // Second pass: semantic analysis
    let mut comments2 = comments.clone();
    let mut semantic_parser = Parser::new(&tokens, &mut comments2, false, arcanum);

    let ast = match semantic_parser.parse() {
        Ok(frame_module) => frame_module,
        Err(parse_error) => {
            eprintln!("Parse error during validation: {}", parse_error.error);
            return false;
        }
    };

    if semantic_parser.had_error() {
        eprintln!(
            "Parser errors during validation: {}",
            semantic_parser.get_errors()
        );
        return false;
    }

    // Validate each system in the frame module
    let mut overall_success = true;

    if ast.systems.is_empty() {
        eprintln!("Warning: No systems found in Frame module");
        return true; // No validation needed
    }

    for system_node in &ast.systems {
        // Create validation context with real AST
        let context = ValidationContext {
            ast: system_node,
            source_code: &source_code,
            file_path: path,
            target_language: target_lang,
            generated_code: None,
            symbol_table: None,
        };

        // Run validation
        let (result, formatted_output) = engine.validate_and_format(context);

        // Print results
        for output in formatted_output {
            println!("{}", output);
        }

        if !result.success {
            overall_success = false;
        }
    }

    overall_success
}
