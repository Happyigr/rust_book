use List::{Cons, Nil};

enum List {
    // cons List is the recursive type of data, because of this we don't know the exact size of it on
    // the compile time, this is the place where the box can be the solution, because the box
    // calculate the size of the variable on the runtime
    // example of a cons list (1, (2, (3, Nil)))
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // this is the new variable box that stores the data not in the stack but in the heap
    // the box contains the link to the data in the stack and the data in the heap
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
