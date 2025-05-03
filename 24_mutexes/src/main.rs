use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    // mutex_syntax();
    multi_ownership(); 
    
}

fn multi_ownership() {
    let counter = Arc::new(Mutex::new(0)); // Atomic version of Rc (thread safe)
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

    println!("Result: {}", *counter.lock().unwrap());

}

fn mutex_syntax() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap(); // You must lock to access a Mutex
        *num = 6;
    } // lock is automatically dropped when num goes out of scope

    println!("m = {:?}", m);
}
