enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!(
        "Amount of the refs after creating the a list {}",
        Rc::strong_count(&a)
    );
    // Rc::clone() only increments the refs count and don't do the deep copy of the list, because of
    // it, it is very fast
    let b = Cons(2, Rc::clone(&a));
    println!(
        "Amount of the refs after creating the b list {}",
        Rc::strong_count(&a)
    );
    {
        let c = Cons(6, Rc::clone(&a));
        println!(
            "Amount of the refs after creating the c list {}",
            Rc::strong_count(&a)
        );
    }
    println!(
        "Amount of the refs after the c list goes out of scope {}",
        Rc::strong_count(&a)
    );
}
