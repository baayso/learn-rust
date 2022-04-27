use minigrep::Argument;
use std::error::Error;
use std::{env, process};

// cargo run searchstring example-filename.txt
fn main() {
    let args: Vec<String> = env::args().collect();

    let argument = Argument::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(argument) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
