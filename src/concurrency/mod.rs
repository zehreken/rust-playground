use std::thread;
use std::time::Duration;

pub fn start_concurrency() {
    // Saving the value of the thread(handle)
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{} from side thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("{} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // Calling join blocks the current thread from exiting

    let v = vec![1, 2, 3];
    let handle2 = thread::spawn(move || {
        for i in 1..5 {
            println!("here is a vector {:?}", v);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..10 {
        println!("Another {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // handle2.join().unwrap();
    // handle.join().unwrap();
}
