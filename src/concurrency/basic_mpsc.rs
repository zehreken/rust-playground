use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

pub fn start() {
    let (tx, rx) = mpsc::channel();

    for _ in 0..8 {
        let tx_t = tx.clone();
        thread::spawn(move || {
            for i in 0..8 {
                let val = format!("message {}", i);
                tx_t.send(val).unwrap();
                thread::sleep(Duration::from_millis(1000));
            }
        });
    }

    for received in rx {
        println!("{}", received);
    }
}
