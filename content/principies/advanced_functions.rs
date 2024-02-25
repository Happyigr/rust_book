// we can pass the functions in the functions!
// fot this idee we have a fn type called function pointer not Fn trait!

fn add_one(x: i32) {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// also we can pass the function to the clousure:
// ...iter.map(ToString::to_string).collect()
// we pass the to_string function to the closure that are implemented in the ToString trait

enum Status {
    Value(u32),
    Stop,
}
// we can also return a closure:
fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    // with this method we can initialize the variable like so:
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}
