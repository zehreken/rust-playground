use crate::rust_book::lib::Config;
use std::env;
use std::process;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = crate::rust_book::lib::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
