use std::env::Args;
use std::{env, process};

use minigrep::Argument;

// cargo run searchstring example-filename.txt
fn main() {
    let args: Args = env::args();

    let argument = Argument::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(argument) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
