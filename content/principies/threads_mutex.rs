// Mutex is the method with which two threads can communicate with each other, and dont do a lot of
// errors this works like a lot of people that wants to speak in one microphone, they need to ask
// the speaker if he had all said or not.
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // here we make the new var with Mutex
    let m = Mutex::new(5);

    {
        // if we want to do something with Mutex value we have to lock this value for our vat
        let mut num = m.lock().unwrap();
        *num = 6;
        // here the lock will be automaticly dropped because the mutex has the Drop trait implemented
    }

    println!("m = {:?}", m);

    // there can be a Deadlock, when to Mutex trying to unlock the Mutex in two threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // we can't use the rc here for multyownership because the rc will not observe the ref
        // counts in concurrency
        // let counter = Rc::clone(&counter);
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
