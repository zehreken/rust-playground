mod cli;
mod tasks;
use cli::{Action::*, CommandLineArgs};
use structopt::StructOpt;
use tasks::Task;

pub fn run() {
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file.expect("Failed to find journal file");

    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }
    .expect("Failed to perform action")
}
