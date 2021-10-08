use crate::frame_c::compiler::Exe;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    /// Path to configuration file.
    #[structopt(short, long)]
    config: Option<PathBuf>,
    /// Generate a default config.yaml file and exit.
    #[structopt(short, long)]
    generate_config: bool,
    /// Path to frame specification file.
    #[structopt(parse(from_os_str))]
    path: PathBuf,
    /// Target language.
    language: String,
}

impl Cli {
    pub fn new(config: Option<PathBuf>, path: PathBuf, language: String) -> Cli {
        Cli {
            config,
            generate_config: false,
            path,
            language,
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
        match exe.write_default_config_file() {
            Ok(()) => {}
            Err(err) => {
                eprintln!("Error generating config.yaml file:\n{}", err.error);
                std::process::exit(err.code);
            }
        }
        return;
    }

    // run the compiler and print output to stdout
    match exe.run_file(&args.config, &args.path, args.language) {
        Ok(code) => {
            println!("{}", code);
        }
        Err(err) => {
            eprintln!("Framec failed with an error:\n{}", err.error);
            std::process::exit(err.code);
        }
    }
}
