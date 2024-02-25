// macros is the function in rust that generates the code
// this is a symplified definition of the vec! macro
// see more in Macros By Example rust refernce
// there are the little book of macros too https://veykril.github.io/tlborm/

// this is declarative macro
#[macro_export]
macro_rules! vec {
    // this is definition is simillar to the match arms this is the 1
    // the $ sign is like a variables and will be replaced with the code that is written in body
    // $x replaced with each expression founded
    // * matches zero or whatever is before the asterics
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// the vec![1,2,3] will be produce this code
//
// {
// let mut temp_vec = Vec::new();
// temp_vec.push(1);
// temp_vec.push(2);
// temp_vec.push(3);
// temp_vec
// }

// this is procedural macro
// this macro acts more like a function and must be declared in the crate
// use proc_macro;
//
// #[some_attribute]
// // TokenStream is the sequence of tokens
// pub fn some_name(input: TokenStream) -> TokenStream {}

pub trait HelloMacro {
    fn hello_macro();
}

// Attribute-like macros generates the code that we want, but with attributes, like procedural
// macro
// example of usage
// #[route(GET, '/')]
// fn index(){}
//
// example of implementing
// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
//

// Function-like macros
// example of usage
// let sql = sql!(SELECT * FROM posts WHERE id=1);
//
// example of implemetation
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {
