use crate::frame_c::compiler::{detect_header_target_annotation, Exe, TargetLanguage};
use crate::frame_c::config::FrameConfig;
use crate::frame_c::v3::multifile_demo::compile_multiple_bodies_demo;
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
    /// Validate-only mode
    validate_only: bool,
    /// Validate (structural) and continue
    validate: bool,
    command: CliCommand,
}

#[derive(Debug, Clone)]
pub enum CliCommand {
    None,
    Init,
    DemoMulti { language: String, files: Vec<PathBuf> },
    DemoProject { language: String, dir: PathBuf, recursive: bool },
    DemoFrame { language: String, file: PathBuf },
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
            .arg(Arg::new("validate").long("validate").help("Run V3 validation before transpile").action(clap::ArgAction::SetTrue))
            .arg(Arg::new("validate-syntax").long("validate-syntax").help("Alias for --validate (compat) ").action(clap::ArgAction::SetTrue))
            .arg(Arg::new("validation-only").long("validation-only").help("Run validation only and exit with status").action(clap::ArgAction::SetTrue))
            .arg(Arg::new("validation-level").long("validation-level").help("Validation level (compat)").num_args(1))
            .arg(Arg::new("validation-format").long("validation-format").help("Validation output format (compat)").num_args(1))
            .subcommand(
                Command::new("demo-multi")
                    .about("V3 demo: compile multiple single-body files (transpile-only)")
                    .arg(
                        Arg::new("language")
                            .long("language").short('l')
                            .value_name("LANG").required(true)
                            .help("Target language: python_3, typescript, csharp, c, cpp, java, rust"),
                    )
                    .arg(
                        Arg::new("file")
                            .value_name("FILE")
                            .help("Input file(s) starting with '{' single native body")
                            .num_args(1..),
                    )
            )
            .subcommand(
                Command::new("demo-project")
                    .about("V3 demo: compile all single-body files in a directory (transpile-only)")
                    .arg(
                        Arg::new("language")
                            .long("language").short('l')
                            .value_name("LANG").required(true)
                            .help("Target language: python_3, typescript, csharp, c, cpp, java, rust"),
                    )
                    .arg(
                        Arg::new("dir")
                            .value_name("DIR")
                            .help("Source directory containing files starting with '{'")
                            .required(true),
                    )
                    .arg(
                        Arg::new("recursive")
                            .long("recursive").short('r')
                            .help("Recurse into subdirectories")
                            .action(clap::ArgAction::SetTrue),
                    )
            )
            .subcommand(
                Command::new("demo-frame")
                    .about("V3 demo: compile a Frame-like file with multiple bodies")
                    .arg(Arg::new("language").long("language").short('l').value_name("LANG").required(true))
                    .arg(Arg::new("file").value_name("FILE").required(true))
            )
            .get_matches();

        let mut has_subcommand = false;
        let command = match matches.subcommand() {
            Some((name, sub)) => {
                has_subcommand = true;
                match name {
                    "init" => CliCommand::Init,
                    "demo-multi" => {
                        let lang = sub.get_one::<String>("language").expect("language required").to_string();
                        let files = sub
                            .get_many::<String>("file")
                            .map(|vals| vals.map(|s| PathBuf::from(s)).collect::<Vec<_>>())
                            .unwrap_or_default();
                        CliCommand::DemoMulti { language: lang, files }
                    }
                    "demo-project" => {
                        let lang = sub.get_one::<String>("language").expect("language required").to_string();
                        let dir = sub.get_one::<String>("dir").map(|s| PathBuf::from(s)).expect("dir required");
                        let recursive = sub.get_flag("recursive");
                        CliCommand::DemoProject { language: lang, dir, recursive }
                    }
                    "demo-frame" => {
                        let lang = sub.get_one::<String>("language").expect("language required").to_string();
                        let file = sub.get_one::<String>("file").map(|s| PathBuf::from(s)).expect("file required");
                        CliCommand::DemoFrame { language: lang, file }
                    }
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
        let validate_only = matches.get_flag("validation-only");
        let validate = matches.get_flag("validate") || matches.get_flag("validate-syntax");

        Cli {
            stdin_flag: stdin,
            path: path_opt,
            language: language_opt,
            multifile,
            output_dir: output_dir_opt,
            debug_output,
            validate_only,
            validate,
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
        CliCommand::DemoMulti { language, files } => {
            // parse language
            let lang = match TargetLanguage::try_from(language) {
                Ok(l) => l,
                Err(e) => { eprintln!("Invalid target language: {}", e); std::process::exit(exitcode::USAGE); }
            };
            let mut inputs: Vec<(String, String)> = Vec::new();
            for p in files {
                match std::fs::read_to_string(&p) {
                    Ok(c) => inputs.push((p.file_name().and_then(|n| n.to_str()).unwrap_or("unknown").to_string(), c)),
                    Err(e) => { eprintln!("Failed to read {}: {}", p.display(), e); std::process::exit(exitcode::NOINPUT); }
                }
            }
            if args.validate || args.validate_only {
                for (name, content) in &inputs {
                    match super::v3::validate_single_body(content, Some(lang)) {
                        Ok(res) => {
                            for issue in res.issues { eprintln!("{}: validation: {}", name, issue.message); }
                            if args.validate_only && !res.ok { std::process::exit(exitcode::DATAERR); }
                        }
                        Err(e) => { eprintln!("{}: validation error: {}", name, e.error); if args.validate_only { std::process::exit(e.code); } }
                    }
                }
                if args.validate_only { return; }
            }
            match compile_multiple_bodies_demo(inputs.iter().map(|(n,c)| (n.as_str(), c.as_str())).collect(), lang) {
                Ok(outputs) => {
                    for (name, code) in outputs {
                        println!("=== file: {} ===\n{}", name, code);
                    }
                }
                Err(e) => { eprintln!("{}", e.error); std::process::exit(e.code); }
            }
            return;
        }
        CliCommand::DemoProject { language, dir, recursive } => {
            let lang = match TargetLanguage::try_from(language) {
                Ok(l) => l,
                Err(e) => { eprintln!("Invalid target language: {}", e); std::process::exit(exitcode::USAGE); }
            };
            if args.validate || args.validate_only {
                // Validate each eligible single-body file
                fn iter_files(dir: &std::path::Path, recursive: bool) -> std::io::Result<Vec<std::path::PathBuf>> {
                    let mut out = Vec::new();
                    fn walk(acc: &mut Vec<std::path::PathBuf>, p: &std::path::Path, recursive: bool) -> std::io::Result<()> {
                        for entry in std::fs::read_dir(p)? { let entry = entry?; let path = entry.path(); if path.is_dir() { if recursive { walk(acc, &path, recursive)?; } } else if path.is_file() { acc.push(path); } }
                        Ok(())
                    }
                    walk(&mut out, dir, recursive)?; Ok(out)
                }
                match iter_files(&dir, recursive) {
                    Ok(files) => {
                        for f in files {
                            if let Ok(content) = std::fs::read_to_string(&f) {
                                let bytes = content.as_bytes(); if bytes.first().copied() != Some(b'{') { continue; }
                                match super::v3::validate_single_body(&content, Some(lang)) {
                                    Ok(res) => { for issue in res.issues { eprintln!("{}: validation: {}", f.display(), issue.message); } if args.validate_only && !res.ok { std::process::exit(exitcode::DATAERR); } }
                                    Err(e) => { eprintln!("{}: validation error: {}", f.display(), e.error); if args.validate_only { std::process::exit(e.code); } }
                                }
                            }
                        }
                    }
                    Err(e) => { eprintln!("walk error: {}", e); if args.validate_only { std::process::exit(exitcode::IOERR); } }
                }
                if args.validate_only { return; }
            }
            match crate::frame_c::v3::multifile_demo::compile_directory_demo(&dir, lang, recursive) {
                Ok(outputs) => {
                    for (path, code) in outputs {
                        println!("=== file: {} ===\n{}", path.display(), code);
                    }
                }
                Err(e) => { eprintln!("{}", e.error); std::process::exit(e.code); }
            }
            return;
        }
        CliCommand::DemoFrame { language, file } => {
            let lang = match TargetLanguage::try_from(language) {
                Ok(l) => l,
                Err(e) => { eprintln!("Invalid target language: {}", e); std::process::exit(exitcode::USAGE); }
            };
            match std::fs::read_to_string(&file) {
                Ok(content) => {
                    if args.validate || args.validate_only {
                        match crate::frame_c::v3::validate_module_demo(&content, lang) {
                            Ok(res) => {
                                for issue in res.issues { eprintln!("{}: validation: {}", file.display(), issue.message); }
                                if args.validate_only { std::process::exit(if res.ok { 0 } else { exitcode::DATAERR }); }
                            }
                            Err(e) => { eprintln!("{}: validation error: {}", file.display(), e.error); if args.validate_only { std::process::exit(e.code); } }
                        }
                    }
                    match crate::frame_c::v3::compile_module_demo(&content, lang) {
                        Ok(code) => { println!("{}", code); }
                        Err(e) => { eprintln!("{}", e.error); std::process::exit(e.code); }
                    }
                }
                Err(e) => { eprintln!("Failed to read {}: {}", file.display(), e); std::process::exit(exitcode::NOINPUT); }
            }
            return;
        }
        CliCommand::None => {}
    }

    let exe = Exe::new();
    // Validation-only pathway (V3 demo; single-body input only)
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
    if args.validate_only || args.validate {
        let path = args.path.clone().expect("file path required");
        if let Ok(content) = std::fs::read_to_string(&path) {
            // If this appears to be a module file (@target present), run module validation; otherwise single-body demo
            let is_module = content.contains("@target ");
            if is_module {
                // Require target language
                let lang = target_language.unwrap_or(TargetLanguage::Python3);
                match super::v3::validate_module_demo(&content, lang) {
                    Ok(res) => {
                        for issue in res.issues { eprintln!("validation: {}", issue.message); }
                        if args.validate_only { std::process::exit(if res.ok { 0 } else { exitcode::DATAERR }); }
                    }
                    Err(e) => {
                        eprintln!("validation error: {}", e.error);
                        if args.validate_only { std::process::exit(e.code); }
                    }
                }
            } else {
                match super::v3::validate_single_body(&content, target_language) {
                    Ok(res) => {
                        for issue in res.issues { eprintln!("validation: {}", issue.message); }
                        if args.validate_only { std::process::exit(if res.ok { 0 } else { exitcode::DATAERR }); }
                    }
                    Err(e) => {
                        eprintln!("validation error: {}", e.error);
                        if args.validate_only { std::process::exit(e.code); }
                    }
                }
            }
        }
    }

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
