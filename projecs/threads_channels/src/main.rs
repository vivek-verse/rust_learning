#![allow(dead_code, unused_imports, unused_variables)]
use crossbeam::channel;
use std::thread;
use std::time::Duration;

fn sleep_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn expensive_sum(v: Vec<i32>) -> i32 {
    sleep_ms(500);
    println!("Child thread: just about finished");
    v.iter().filter(|&x| x % 2 == 0).map(|x| x * x).sum()
}

fn main() {
    let my_vector = vec![2, 5, 1, 0, 4, 3];

    let handle = thread::spawn(|| expensive_sum(my_vector));

    for letter in vec!["a", "b", "c", "d", "e", "f"] {
        println!("Main thread: Processing the letter '{}'", letter);
        sleep_ms(200);
    }

    let result = handle.join();
    let sum = result.unwrap_or(0);
    println!("The child thread's expensive sum is {}", sum);

    let (tx, rx) = channel::unbounded();

    let tx2 = tx.clone();

    let handle_a = thread::spawn(move || {
        sleep_ms(0);
        tx2.send("Thread A: 1").unwrap();
        sleep_ms(200);
        tx2.send("Thread A: 2").unwrap();
    });

    sleep_ms(100);

    let handle_b = thread::spawn(move || {
        sleep_ms(0);
        tx.send("Thread B: 1").unwrap();
        sleep_ms(200);
        tx.send("Thread B: 2").unwrap();
    });

    for msg in rx {
        println!("Main thread: Received {}", msg);
    }

    handle_a.join().unwrap();
    handle_b.join().unwrap();

    let (tx1, rx1) = channel::unbounded();

    let rx2 = rx1.clone();

    let handle_c = thread::spawn(move || {
        for msg in rx2 {
            println!("Thread A received: {}", msg);
        }
    });

    let handle_d = thread::spawn(move || {
        for msg in rx1 {
            println!("Thread B received: {}", msg);
        }
    });

    for i in 0..50 {
        tx1.send(i).unwrap();
        sleep_ms(20);
    }

    drop(tx1);
    handle_c.join().unwrap();
    handle_d.join().unwrap();

    println!("Main thread: Exiting.")
}
