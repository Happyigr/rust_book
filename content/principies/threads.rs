// when the main thread is compleated other threaids will be automaticly stopped
// if we want not, that the thread will be stopped, we can make a variable with thread
// and then start it with join()
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    let v = vec![1, 2, 3, 4];
    // with move keyword we tell the clousure to take the ownership of the value rather then to
    // borrow it
    let handle = thread::spawn(move || {
        println!("Here's the vector {:?}", v);
    });

    for i in 1..9 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_secs(1));
    }

    // Code that runs after this join() will waiting this thread to end, and only then it will
    // executed
    handle.join().unwrap();

    // if we want, that two threads communicate with each other, we need to create a channel. We
    // should think about channel like a stream of water.

    // we cant make a channel with unknown sender and reciever, because then rust can't know which
    // data it will sending
    // we can have a lot sender but only one reciever,
    // tx is for transmitter and rx for reciever
    let (tx, rx) = mpsc::channel();

    // the spawned thread needs to own the transmitter of the channel, if it want to send value in
    // it
    let tx1 = tx.clone();
    thread::spawn(move || {
        let val = vec![
            String::from("Hi, "),
            String::from("statring a messaging"),
            String::from("this is a new thread messaging"),
            String::from("bye"),
        ];
        for v in val {
            // send returns the result of sending, Result<>
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // we cant do so, because of the ownership rules, that send method gets, when sends a value
        // println!("val is {val}");
    });

    thread::spawn(move || {
        let val = vec![
            String::from("another +++"),
            String::from("$thread"),
            String::from("%huhu"),
            String::from("(njansdjn)"),
        ];
        for v in val {
            // send returns the result of sending, Result<>
            tx.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // we cant do so, because of the ownership rules, that send method gets, when sends a value
        // println!("val is {val}");
    });

    for recieved in rx {
        println!("recieved: {recieved}");
    }
}
