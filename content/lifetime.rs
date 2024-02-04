use std::fmt::Display;

// this is a struct that livrs so long as the reference, that would gave to this struct
struct SomeStruct<'a> {
    part: &'a str,
}

impl<'a> SomeStruct<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    // all strings references have the static lifetime (living to the end of the programm)
    // this str will live for the entire programm
    let str1: &'static str = "koko";
    let str1 = String::from("abcd");
    let i = SomeStruct { part: &str1 };
    let result;
    // because we have specified the lifetime, if we would have here another scope, we will get an
    // error, because our result ref lives as long as the smallest lifetime of the both of given
    // variables
    // {
    let str2 = String::from("xyz");
    result = longest(str1.as_str(), str2.as_str());
    // }
    println!("the longest string is {}", result);
}

// there are 3 rules, after which compiler will ask you to give types explicit in a funciton, this
// can be readed in Lifetime elysion rust book chapter 10

// this means that the reference will live so long how the smallest lifetime of both variables,
// this is so because we dont know which lifetime we should use(of x or of y)
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // this will be not compile, because the borrow checker dont know how long the reference will
    // live (lifetimes) so we need to write it extra with lifetimes
    // fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_annotation<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcment: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// we have some functions that are working with references, but we dont need to write it specifily,
// this called Lifetime Elision (the rule that the compiler knows hot to manage)

fn first_word(s: &str) -> &str {
    // returns bytes
    &s[0..1]
}
