use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    let query = &args[2];
    let filename = &args[3];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
