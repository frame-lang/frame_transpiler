use crate::frame_c::compiler::{Exe, TargetLanguage};
use crate::frame_c::config::FrameConfig;
use std::convert::TryFrom;
use std::path::PathBuf;
use structopt::StructOpt;

/// Command line arguments to the `framec` executable.
#[derive(StructOpt)]
pub struct Cli {
    /// Path to configuration file.
    #[structopt(short, long)]
    config: Option<PathBuf>,

    /// Generate a default config.yaml file and exit.
    #[structopt(short, long)]
    generate_config: bool,

    /// Path to frame specification file.
    #[structopt(parse(from_os_str), required_unless = "generate-config")]
    path: Option<PathBuf>,

    /// Target language.
    #[structopt(required_unless = "generate-config")]
    language: Option<String>,
}

impl Cli {
    pub fn new(config: Option<PathBuf>, path: PathBuf, language: String) -> Cli {
        Cli {
            config,
            generate_config: false,
            path: Some(path),
            language: Some(language),
        }
    }
}

/// Parse command-line arguments and run the compiler.
pub fn run() {
    run_with(Cli::from_args());
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
            Ok(lang) => lang,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(exitcode::USAGE);
            }
        },
        None => {
            eprintln!("No target language specified.");
            std::process::exit(exitcode::USAGE);
        }
    };

    // run the compiler and print output to stdout
    match exe.run_file(&args.config, &args.path.unwrap(), Some(target_language)) {
        Ok(code) => {
            println!("{}", code);
        }
        Err(err) => {
            eprintln!("Framec failed with an error:\n{}", err.error);
            std::process::exit(err.code);
        }
    }
}
