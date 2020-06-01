use std::sync::{Arc, Mutex};
use std::thread;

pub fn start() {
    // simple();
    multiple_threads();
}

fn simple() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

fn multiple_threads() {
    let vec = Arc::new(Mutex::new(vec![]));
    let mut handles = vec![];

    for i in 0..10 {
        let vec = Arc::clone(&vec);
        let handle = thread::spawn(move || {
            let mut v = vec.lock().unwrap();

            v.push(i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {:?}", *vec.lock().unwrap());
}
