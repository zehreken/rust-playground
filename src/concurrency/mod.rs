mod basic_mpsc;
mod basic_threads;

pub fn start_concurrency() {
    // basic_threads::start();
    basic_mpsc::start();
}
