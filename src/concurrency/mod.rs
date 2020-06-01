mod basic_mpsc;
mod basic_threads;
mod shared_state;

pub fn start_concurrency() {
    // basic_threads::start();
    // basic_mpsc::start();
    shared_state::start();
}
