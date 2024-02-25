// with usnafe code we can do 5 things:
// Dereference a raw pointer
// Call an unsafe function or method
// Access or modify a mutable static variable
// implement an unsafe trait
// Access fields of union S

use std::slice;

// we can also use the other languages in rust with the extern code word, but this will be always
// unsafe, because other languages dont have the rules of the rust language
extern "C" {
    fn abs(input: i32) -> i32;
}

// we can also export functions from rust to use them in other languages
// no_mangle means, that the compiler will not change the names from rust names, but give it to
// compiler???
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// global variables are the variables with static lifetime, and due to the ownership rules, this is
// a problem to change this variable
static HELLO_WORLD: &str = "Hello, global";
// accesing the immutable static var is safe, but accessing the mutable static var is unsafe
static mut COUNTER: usize = 0;

fn add_to_count(inc: usize) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    let mut num = 5;

    // the asteriks isn't a dereference operator , but a name of the type
    // we can also give an exact address to the pointer
    // let adress = 0x012345usize;
    // because of it we can make a pointer to everything, but we can't dereference it
    // with the raw pointers we can create the immutable and a mutable ref to the var, what can be
    // the create the data races.
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        // we need to wrap the unsafe functions in this unsafe block too
        // but we can make a safe abstraction to the unsafe function
        println!("r1: {}, r2: {}", *r1, *r2);
        println!("abs(-3) is {}", abs(-3));
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);
}
// so could the safe answer see, but this will be not compiling, due to the ownership rules
// fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = values.len();
//
//     assert!(mid <= len);
//
//     (&mut values[..mid], &mut values[mid..])
// }

// this is the safe abstraction of the unsafe code, we should do it when we know, that nothing bad
// gonna happen, but the rust doesn't understand this
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            // the slices is the pointer to some data and the length of the slice
            slice::from_raw_parts_mut(ptr, len),
            slice::from_raw_parts_mut(ptr.add(mid), len),
        )
    }
}
