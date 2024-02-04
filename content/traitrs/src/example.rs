use crate::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("myName"),
        content: String::from("Hi, this is my first tweet"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
