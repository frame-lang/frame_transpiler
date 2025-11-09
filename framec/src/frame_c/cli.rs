use crate::frame_c::compiler::{detect_header_target_annotation, Exe, TargetLanguage};
use crate::frame_c::config::FrameConfig;
use clap::{Arg, Command};
use std::convert::TryFrom;
use std::path::PathBuf;

pub struct Cli {
    stdin_flag: bool,
    path: Option<PathBuf>,
    language: Option<String>,
    multifile: bool,
    output_dir: Option<PathBuf>,
    debug_output: bool,
    command: CliCommand,
}

#[derive(Debug, Clone)]
pub enum CliCommand {
    None,
    Init,
}

impl Cli {
    pub fn new() -> Cli {
        let matches = Command::new("framec")
            .version(env!("FRAME_VERSION"))
            .about("Frame language transpiler (V3 architecture rebuild in progress)")
            .subcommand_required(false)
            .arg_required_else_help(false)
            .subcommand(Command::new("init").about("Initialize a new Frame project with frame.toml").arg(Arg::new("name").help("Project name").value_name("NAME").index(1)))
            .arg(Arg::new("FILE-PATH").help("File path").value_name("FILE").index(1))
            .arg(Arg::new("language").value_name("LANG").long("language").short('l').help("Target language (python_3, typescript, graphviz, llvm)").num_args(1))
            .arg(Arg::new("multifile").long("multifile").short('m').help("Enable multi-file project compilation").action(clap::ArgAction::SetTrue))
            .arg(Arg::new("output-dir").long("output-dir").short('o').help("Output directory for generated files (multi-file mode)").value_name("DIR").num_args(1))
            .arg(Arg::new("debug-output").long("debug-output").help("Generate JSON output with transpiled code and source map").action(clap::ArgAction::SetTrue))
            .get_matches();

        let mut has_subcommand = false;
        let command = match matches.subcommand() {
            Some((name, _)) => {
                has_subcommand = true;
                match name {
                    "init" => CliCommand::Init,
                    _ => CliCommand::None,
                }
            }
            None => CliCommand::None,
        };

        let mut stdin = false;
        let mut path_opt = None;
        if !has_subcommand && matches.contains_id("FILE-PATH") {
            let file_path = matches.get_one::<String>("FILE-PATH");
            path_opt = file_path.map(|file_path| PathBuf::from(file_path.clone()));
        } else if !has_subcommand {
            stdin = true;
        }

        let language_opt = matches.get_one::<String>("language").map(|s| s.clone());
        let multifile = matches.get_flag("multifile");
        let output_dir_opt = matches.get_one::<String>("output-dir").map(|s| PathBuf::from(s.clone()));
        let debug_output = matches.get_flag("debug-output");

        Cli {
            stdin_flag: stdin,
            path: path_opt,
            language: language_opt,
            multifile,
            output_dir: output_dir_opt,
            debug_output,
            command,
        }
    }
}

impl Default for Cli {
    fn default() -> Self {
        Cli::new()
    }
}

pub fn run() {
    run_with(Cli::new());
}

pub fn run_with(args: Cli) {
    match args.command {
        CliCommand::Init => {
            handle_init_command();
            return;
        }
        CliCommand::None => {}
    }

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

    if args.stdin_flag {
        match exe.run_stdin(target_language) {
            Ok(code) => println!("{}", code),
            Err(err) => {
                eprintln!("{}", err.error);
                std::process::exit(err.code);
            }
        }
    } else {
        let path = args.path.unwrap();
        let result = if args.debug_output {
            exe.run_file_debug(&path, target_language)
        } else if args.multifile {
            exe.run_multifile(&path, target_language, args.output_dir)
        } else {
            exe.run_file(&path, target_language)
        };

        match result {
            Ok(code) => println!("{}", code),
            Err(err) => {
                eprintln!("{}", err.error);
                std::process::exit(err.code);
            }
        }
    }
}

fn handle_init_command() {
    use std::env;
    use std::fs;

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let config_path = current_dir.join("frame.toml");

    if config_path.exists() {
        eprintln!("frame.toml already exists in this directory");
        std::process::exit(exitcode::CANTCREAT);
    }

    let project_name = current_dir.file_name().and_then(|n| n.to_str()).map(|s| s.to_string());

    match FrameConfig::create_default(&config_path, project_name.as_deref()) {
        Ok(_) => {
            println!("Created frame.toml");
            let src_dir = current_dir.join("src");
            if !src_dir.exists() {
                fs::create_dir(&src_dir).expect("Failed to create src directory");
                println!("Created src/");
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
        }
        Err(e) => {
            eprintln!("Failed to create frame.toml: {}", e);
            std::process::exit(exitcode::IOERR);
        }
    }
}

