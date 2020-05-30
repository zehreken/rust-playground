use std::thread;
use std::time::Instant;

pub fn start_basic_threads() {
    let now = Instant::now();
    single_thread();
    let diff = Instant::now() - now;
    println!("{:?}", diff);

    let now = Instant::now();
    multiple_threads();
    let diff = Instant::now() - now;
    println!("{:?}", diff);
}

fn single_thread() {
    for _ in 0..8 {
        for _ in 0..8 {
            expensive_procedure();
        }
    }
}

fn multiple_threads() {
    let mut handles = vec![];
    for _ in 0..8 {
        handles.push(thread::spawn(move || {
            for _ in 0..8 {
                expensive_procedure();
            }
        }));
    }

    for h in handles {
        let v = h.join().unwrap(); // Calling join blocks the current thread from exiting
    }
}

fn expensive_procedure() -> f32 {
    const TRY_COUNT: i32 = 1_000_000;
    let mut sum = 0.0;
    for i in 0..TRY_COUNT {
        sum += (i as f32).sin();
    }

    sum
}
