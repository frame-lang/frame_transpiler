use crate::frame_c::compiler::{Exe, TargetLanguage};
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
}

impl Cli {
    pub fn new() -> Cli {
        let matches = Command::new("framec")
            .version("0.30.0")
            .about("Frame language transpiler")
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
            .get_matches();

        let mut stdin = false;
        let mut path_opt = None;
        if matches.contains_id("FILE-PATH") {
            let file_path = matches.get_one::<String>("FILE-PATH");
            path_opt = file_path.map(|file_path| PathBuf::from(file_path.clone()));
        } else {
            stdin = true;
        }

        let language = matches.get_one::<String>("language");
        let language_opt = language.map(|lang| lang.clone());

        Cli {
            stdin_flag: stdin,
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
        match exe.run_file(&args.path.unwrap(), target_language) {
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
