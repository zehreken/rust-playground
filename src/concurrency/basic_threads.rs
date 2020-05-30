use std::thread;
use std::time::Instant;

pub fn start_basic_threads() {
    let now = Instant::now();
    let sum = single_thread();
    let diff = Instant::now() - now;
    println!("{} in {:?}", sum, diff);

    let now = Instant::now();
    let sum = multiple_threads();
    let diff = Instant::now() - now;
    println!("{} in {:?}", sum, diff);
}

fn single_thread() -> f32 {
    let mut sum = 0.0;
    for _ in 0..8 {
        for _ in 0..8 {
            sum += expensive_procedure();
        }
    }

    sum
}

fn multiple_threads() -> f32 {
    let mut sum = 0.0;
    let mut handles = vec![];
    for _ in 0..8 {
        handles.push(thread::spawn(move || {
            let mut v = vec![];
            for _ in 0..8 {
                v.push(expensive_procedure());
            }
            v
        }));
    }

    for h in handles {
        let v = h.join().unwrap(); // Calling join blocks the current thread from exiting
        for i in v {
            sum += i;
        }
    }

    sum
}

fn expensive_procedure() -> f32 {
    const TRY_COUNT: i32 = 1_000_000;
    let mut sum = 0.0;
    for i in 0..TRY_COUNT {
        sum += (i as f32).sin();
    }

    sum
}
