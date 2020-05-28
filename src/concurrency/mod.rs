use std::thread;
use std::time::{Duration, Instant};

pub fn start_concurrency() {
    let now = Instant::now();
    single_thread();
    let diff = Instant::now() - now;
    println!("{:?}", diff);

    let now = Instant::now();
    simple_threads();
    let diff = Instant::now() - now;
    println!("{:?}", diff);
}

fn single_thread() {
    for j in 0..8 {
        for i in 0..8 {
            expensive_procedure();
        }
    }
}

fn simple_threads() {
    let mut handles = vec![];
    for j in 0..8 {
        // let mut v = vec![];
        handles.push(thread::spawn(move || {
            for i in 0..8 {
                expensive_procedure();
            }
        }));
    }

    for h in handles {
        let v = h.join().unwrap(); // Calling join blocks the current thread from exiting
    }
}

fn expensive_procedure() -> f32 {
    const TRY_COUNT: i32 = 10_000_000;
    let mut sum = 0.0;
    for i in 0..TRY_COUNT {
        sum += (i as f32).sin();
    }

    sum
}
