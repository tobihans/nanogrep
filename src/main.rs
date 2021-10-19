use std::{env, process};
use nanogrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = nanogrep::run(config) {
        eprintln!("An error occured during processing: {}", e);
        process::exit(1);
    };
}


