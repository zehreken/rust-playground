use std::thread;
use std::time::Duration;

pub fn start_concurrency() {
    let mut handles = vec![];
    for j in 0..8 {
        let mut v = vec![];
        handles.push(thread::spawn(move || {
            for i in 0..8 {
                v.push(j * 8 + i);
            }
            v
        }));
    }

    for h in handles {
        let v = h.join().unwrap(); // Calling join blocks the current thread from exiting
        println!("{:?}", v);
    }
}
