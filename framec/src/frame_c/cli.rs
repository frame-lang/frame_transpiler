use crate::frame_c::compiler::{Exe, TargetLanguage};
use crate::frame_c::config::FrameConfig;
use std::convert::TryFrom;
use std::path::PathBuf;
// use structopt::StructOpt;
use clap::Arg;

/// Command line arguments to the `framec` executable.
//#[derive(StructOpt)]
pub struct Cli {
    /// Path to configuration file.
    config: Option<PathBuf>,

    /// Generate a default config.yaml file and exit.
    generate_config: bool,

    /// Stdin flag. Mutually exclusive with path
    stdin_flag: bool,

    /// Path to frame specification file.
    path: Option<PathBuf>,

    /// Target language.
    language: Option<String>,
}

impl Cli {
    pub fn new() -> Cli {
        let matches = clap::Command::new("framec")
            .version("0.8.0")
            .about("Says hello")
            .arg(
                Arg::new("GENERATE-CONFIG")
                    .long("generate-config")
                    .required(false)
                    .help("Generate config flag"),
            )
            .arg(
                Arg::new("CONFIG-PATH")
                    .long("config")
                    .takes_value(true)
                    .help("Config path")
                    .required(false),
            )
            .arg(Arg::new("FILE-PATH").help("File path"))
            .arg(
                Arg::new("language")
                    .takes_value(true)
                    .long("language")
                    .short('l')
                    .help("Target language"),
                //                    .required_unless_present("GENERATE-CONFIG"),
            )
            .get_matches();

        let generate_config = matches.is_present("GENERATE-CONFIG");

        let config_path_str_opt = matches.value_of("CONFIG-PATH");
        let config_path_pathbuf_opt = config_path_str_opt.map(PathBuf::from);

        let mut stdin = false;
        let mut path_opt = None;
        if matches.is_present("FILE-PATH") {
            let file_path = matches.value_of("FILE-PATH");
            // path_opt = match file_path {
            //     Some(file_path) => Some(PathBuf::from(file_path.to_string())),
            //     None => None,
            // };
            path_opt = file_path.map(|file_path| PathBuf::from(file_path.to_string()));
        } else {
            stdin = true;
        }

        let language = matches.value_of("language");

        let language_opt = language.map(|lang| lang.to_string());

        Cli {
            stdin_flag: stdin,
            config: config_path_pathbuf_opt,
            generate_config,
            path: path_opt,
            language: language_opt,
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
    let exe = Exe::new();

    // generate config file, if requested, then exit
    if args.generate_config {
        match FrameConfig::write_default_yaml_file() {
            Ok(()) => {}
            Err(err) => {
                eprintln!("Error generating config.yaml file:\n{}", err.error);
                std::process::exit(err.code);
            }
        }
        return;
    }

    let target_language = match args.language {
        Some(lang_str) => match TargetLanguage::try_from(lang_str) {
            Ok(lang) => Some(lang),
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(exitcode::USAGE);
            }
        },
        None => {
            // eprintln!("No target language specified.");
            // std::process::exit(exitcode::USAGE);
            None
        }
    };

    // run the compiler and print output to stdout
    if args.stdin_flag {
        match exe.run_stdin(&args.config, target_language) {
            // match exe.run_stdin(&args.config, args.language.unwrap()) {
            Ok(code) => {
                println!("{}", code);
            }
            Err(err) => {
                eprintln!("Framec failed with an error:\n{}", err.error);
                std::process::exit(err.code);
            }
        }
    } else {
        match exe.run_file(&args.config, &args.path.unwrap(), target_language) {
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
