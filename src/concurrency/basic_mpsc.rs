use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

pub fn start() {
    // single_sender_sample_1();
    // single_sender_sample_2();
    sender_receiver_different_threads();
    // multiple_senders();
}

fn sender_receiver_different_threads() {
    let (tx, rx) = mpsc::channel();

    let sender_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(1000));
        tx.send("message");
    });

    let receiver_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(2000));
        println!("{}", rx.recv().unwrap());
    });

    sender_handle.join().unwrap();
}

fn single_sender_sample_1() {
    let (tx, rx) = mpsc::channel();

    let tx_c = tx.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(1000));
        tx_c.send("message");
    });

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(2000));
        drop(tx);
    });

    // This loop blocks the thread
    // The drop call in the above thread removes the block
    for received in rx {
        println!("{}", received);
    }

    println!("not blocked");
}

fn single_sender_sample_2() {
    let (tx, rx) = mpsc::channel();

    tx.send("message");

    // This loop blocks the thread
    for received in rx {
        println!("{}", received);
    }
}

fn multiple_senders() {
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
