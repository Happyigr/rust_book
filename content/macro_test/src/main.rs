use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancake(usize);

fn main() {
    let pan = Pancake(2);
    Pancake::hello_macro();
}
