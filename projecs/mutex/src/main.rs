use std::sync::{Arc, Mutex};
use std::thread;
fn main() {
    /*
        Mutex rules
        1. Aquire a lock before data access
        2. Release lock after you are done with the data
        Locks are automatically dropped by the compiler, no need to do it yourself!
    */
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result : {}", *counter.lock().unwrap());
}
