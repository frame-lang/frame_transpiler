use crate::frame_c::compiler::{Exe, TargetLanguage};
use crate::frame_c::config::FrameConfig;
use std::convert::TryFrom;
use std::path::PathBuf;
use clap::{Arg, Command};

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
    
    /// Subcommand (build, init, etc.)
    command: Option<String>,
}

impl Cli {
    pub fn new() -> Cli {
        let matches = Command::new("framec")
            .version("0.30.0")
            .about("Frame language transpiler")
            .subcommand_required(false)
            .arg_required_else_help(false)
            .subcommand(
                Command::new("build")
                    .about("Build project using frame.toml configuration")
            )
            .subcommand(
                Command::new("init")
                    .about("Initialize a new Frame project with frame.toml")
                    .arg(Arg::new("name")
                        .help("Project name")
                        .value_name("NAME")
                        .index(1))
            )
            .arg(Arg::new("FILE-PATH")
                .help("File path")
                .value_name("FILE")
                .index(1))
            .arg(
                Arg::new("language")
                    .value_name("LANG")
                    .long("language")
                    .short('l')
                    .help("Target language")
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

        Cli {
            stdin_flag: stdin,
            path: path_opt,
            language: language_opt,
            multifile,
            output_dir: output_dir_opt,
            config: config_opt,
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
                eprintln!("Unknown command: {}", command);
                std::process::exit(exitcode::USAGE);
            }
        }
    }

    // Original transpiler behavior
    let exe = Exe::new();

    let target_language = match args.language {
        Some(lang_str) => match TargetLanguage::try_from(lang_str) {
            Ok(lang) => Some(lang),
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(exitcode::USAGE);
            }
        },
        None => None,
    };

    // run the compiler and print output to stdout
    if args.stdin_flag {
        match exe.run_stdin(target_language) {
            Ok(code) => {
                println!("{}", code);
            }
            Err(err) => {
                eprintln!("Framec failed with an error:\n{}", err.error);
                std::process::exit(err.code);
            }
        }
    } else {
        let path = args.path.unwrap();
        let result = if args.multifile {
            exe.run_multifile(&path, target_language, args.output_dir)
        } else {
            exe.run_file(&path, target_language)
        };
        
        match result {
            Ok(code) => {
                println!("{}", code);
            }
            Err(err) => {
                eprintln!("Framec failed with an error:\n{}", err.error);
                std::process::exit(err.code);
            }
        }
    }
}

/// Handle the 'init' subcommand to create a new Frame project
fn handle_init_command() {
    use std::fs;
    use std::env;
    
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let config_path = current_dir.join("frame.toml");
    
    if config_path.exists() {
        eprintln!("Error: frame.toml already exists in this directory");
        std::process::exit(exitcode::CANTCREAT);
    }
    
    // Get project name from directory name
    let project_name = current_dir.file_name()
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
                println!("Build successful! Output written to {}/", config.build.output_dir.display());
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
