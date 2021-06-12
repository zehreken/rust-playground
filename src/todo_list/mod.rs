mod cli;
mod tasks;
use structopt::StructOpt;

pub fn run() {
    println!("{:#?}", cli::CommandLineArgs::from_args());
}
