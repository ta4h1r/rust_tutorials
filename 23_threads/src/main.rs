use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // thread_blocking();
    // thread_move();
    // thread_channels();
    multi_tx_single_rx();
}

fn multi_tx_single_rx() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn thread_channels() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap(); // recv blocks main thread execution until data is received
    println!("Got: {}", received);
}

fn thread_move() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        // We must 'move' ownership of v into the closure, so that it does not
        // outlive the main scope
        println!("Here's a vector {:?}", v);
    });

    handle.join().unwrap();
}

fn thread_blocking() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // blocks main thread until spawned thread finishes
}
