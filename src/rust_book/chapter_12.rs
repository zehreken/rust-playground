use crate::rust_book::lib::Config;
use std::env;
use std::process;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = crate::rust_book::lib::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
save, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["save, fast, productive."],
            crate::rust_book::lib::search(query, contents)
        );
    }

    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            crate::rust_book::lib::search_case_insensitive(query, contents)
        );
    }
}
