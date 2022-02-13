use crate::frame_c::compiler::Exe;
use crate::frame_c::config::FrameConfig;
use std::path::PathBuf;
// use structopt::StructOpt;
use clap::{App, Arg};
use core::option;

/// Command line arguments to the `framec` executable.
//#[derive(StructOpt)]
pub struct Cli {

    /// Path to configuration file.
//    #[structopt(short, long)]
    config: Option<PathBuf>,

    /// Generate a default config.yaml file and exit.
//    #[structopt(short, long)]
    generate_config: bool,

    /// Stdin flag. Mutually exclusive with path

    stdin_flag:bool,

    /// Path to frame specification file.
//    #[structopt(parse(from_os_str), required_unless = "generate-config")]
    path: Option<PathBuf>,

    /// Target language.
//    #[structopt(required_unless = "generate-config")]
    language: Option<String>,
}

impl Cli {
    pub fn new() -> Cli {
//    pub fn new(config: Option<PathBuf>, path: PathBuf, language: String) -> Cli {
        let app = App::new("framec")
            .version("7.5")
            .about("Says hello");


        let generate_config_arg = Arg::new("generate-config")
//            .takes_value(true)
            .long("generate-config")
            .required(false)
            .help("Generate config flag");
        let app = app.arg(generate_config_arg);


        let config_path_arg = Arg::new("config")
            .long("config")
            .takes_value(true)
            .help("Config path")
            .required(false);
        let app = app.arg(config_path_arg);

        let file_path_arg = Arg::new("FILE_PATH")
//            .takes_value(true)
            .help("File path");
        let app = app.arg(file_path_arg);

        let language_arg = Arg::new("language")
            .takes_value(true)
            .long("language")
            .short('l')
            .help("Target language")
            .required_unless_present("generate-config");

        let app = app.arg(language_arg);

        // extract the matches
        let matches = app.get_matches();

        let generate_config = matches.is_present("generate-config");

        let config_path_str_opt = matches.value_of("config");
        let config_path_pathbuf_opt = match config_path_str_opt {
            Some(config_path_str) => Some(PathBuf::from(config_path_str)),
            None => None
        };

        let mut stdin = false;
        let mut path_opt = None;
        if matches.is_present("FILE_PATH") {
            let file_path = matches.value_of("FILE_PATH");
            path_opt = match file_path {
                Some(file_path) => Some(PathBuf::from(file_path.to_string())),
                None => None,
            };

        } else {
            stdin = true;
        }

        let language = matches.value_of("language");

        let language_opt = match language {
            Some(lang) => Some(lang.to_string()),
            None => None,
        };

        // let str = file_path.to_string();
        // let file_path_buf = PathBuf::from(file_path.to_string());

        Cli {
            stdin_flag: stdin,
            config:config_path_pathbuf_opt,
            generate_config,
            path: path_opt,
            language: language_opt,
        }
    }
}

/// Parse command-line arguments and run the compiler.
pub fn run() {


    run_with(Cli::new());
}

/// Run `framec` with the given CLI options.
pub fn run_with( args: Cli) {
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

    // run the compiler and print output to stdout
    if args.stdin_flag {
        match exe.run_stdin( &args.config,  args.language.unwrap()) {
            Ok(code) => {
                println!("{}", code);
            }
            Err(err) => {
                eprintln!("Framec failed with an error:\n{}", err.error);
                std::process::exit(err.code);
            }
        }
    } else {
        match exe.run_file( &args.config, &args.path.unwrap(), args.language.unwrap()) {
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
