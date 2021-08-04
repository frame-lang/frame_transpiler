use framec::frame_c::compiler::Exe;
use framec::frame_c::utils::*;
use std::fs;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    language: String,
}

fn main() {
    let args = Cli::from_args();

    if let Err(run_error) = run_file(&args.path, &args.language) {
        eprintln!("Error reading file:\n{}", run_error.error);
        std::process::exit(run_error.code)
    };
    // let args: Vec<String> = env::args().collect();
    //
    // match args.len() {
    //     1 =>  println!("Error - missing parameters."),
    //     3 => {
    //         if let Err(e) = run_file(&args[1], &args[2]) {
    //             eprintln!("Error reading file: {}", e);
    //         };
    //     }
    //     _ => println!("Error - invalid number of arguments. Expected 2."),
    // }
}

/* --------------------------------------------------------------------- */

pub fn run_file(filename: &std::path::PathBuf, output_format: &String) -> Result<(), RunError> {
    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(err) => {
            let run_err = RunError::new(exitcode::NOINPUT, &*err.to_string());
            return Err(run_err);
        }
    };
    Exe::debug_print(&format!("{}", &contents));
    let frame_c = Exe::new();
    let run_result = frame_c.run(contents, output_format.clone());
    match run_result {
        Ok(code) => {
            println!("{}", code);
            Ok(())
        }
        Err(run_err) => Err(run_err),
    }
}
