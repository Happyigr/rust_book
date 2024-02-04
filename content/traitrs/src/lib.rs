use std::fmt::Display;
mod example;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // standart behavior
        format!("Read more from {}...", self.summarize_author())
    }
}

// so we can have only the types that have implemented the Summary trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// we declare a type that had implemented the summary trait
pub fn multiple_notify<T: Summary>(item1: &T, item2: &T) {
    println!(
        "Breaking News! {}, {}",
        item1.summarize(),
        item2.summarize()
    );
}

pub fn notify_two_types<T: Summary + Display>(item: &T) {
    item.summarize();
}

pub fn other_notify_with_two_types<T, U>(item1: &T, item2: &U)
where
    T: Summary + Display,
    U: Summary,
{
    item1.summarize();
    println!("Some info: {}", item2.summarize());
}

// but so we cant return two type in one function (Tweet and NewsArticle)
pub fn return_impl_type() -> impl Summary {
    Tweet {
        username: String::from("happyigr"),
        content: String::from("Hi i\'m happy"),
        reply: true,
        retweet: false,
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// we can implement the trait only for a type that implements another trait like this:
// impl<T: Display> Summary for T {...}

// for a standart behavior of trait we should dont implement the trait\, that we want to have the
// standart behavior:
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}
// impl Summary for NewsArticle {
//    fn summarize_author(&self) -> String {
//       format!("@{}", self.author)
//     }
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
