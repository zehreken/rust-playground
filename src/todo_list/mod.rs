mod cli;
use structopt::StructOpt;

pub fn run() {
    cli::CommandLineArgs::from_args();
}
