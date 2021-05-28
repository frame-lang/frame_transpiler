
use std::{env,fs};
use std::io::Error;
use framec::frame_c::compiler::Exe;

fn main() {

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 =>  println!("Error - missing parameters."),
        3 => {
            if let Err(e) = run_file(&args[1], &args[2]) {
                eprintln!("Error reading file: {}", e);
            };
        }
        _ => println!("Error - invalid number of arguments. Expected 2."),
    }
}


/* --------------------------------------------------------------------- */

pub fn run_file(filename:&String,output_format:&String) -> Result<(), Error> {

    let contents = fs::read_to_string(filename)?;
    Exe::debug_print(&format!("{}", &contents));
    let frame_c = Exe::new();
    let output = frame_c.run(contents,output_format.clone());
    println!("{}", output);

    Ok(())
}
