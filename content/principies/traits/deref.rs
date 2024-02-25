use std::ops::Deref;

struct MyBox<T>(T);

// for mutability see DerefMut
impl<T> Deref for MyBox<T> {
    type Target = T;

    // in this function we created the function that returns the ref to the first value of the
    // tuple in our case the ref to the value x in the MyBox struct
    fn deref(&self) -> &Self::Target {
        // .0 accesses the first value of the tuple
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {
    let x = 5;
    // here we stored the ref to the value x
    let y = &x;
    // here we have make a copy of an variable x in heap and stored the ref to it in variable y
    let y = Box::new(x);
    let y2 = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *(y2.deref()));

    let name = MyBox::new(String::from("Rust"));
    // Deref coercion
    // this works, beecause of the Deref trait, that we have implemented, so that the rust knows
    // how to turn String to &str
    hello(&name);
}

fn hello(name: &str) {
    println!("Hello {}!", name);
}
