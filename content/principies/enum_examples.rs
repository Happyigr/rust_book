#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("The quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
        // all other values that will be catchen(here are no such a values)
        // other => None,
        // if we dont use a variable we can use _, that says, that the variable will be unused
        // _ => None,
        // or if we want to do nothing too
        // _ => (),
    }
}

fn main() {
    let config_max = Some(3u8);
    // this ignores all other patterns, and run code only when Some(max) = config_max
    if let Some(max) = config_max {
        println!("Maximum is configured to be {}.", max);
    }
    let qrtala = Coin::Quarter(UsState::Alabama);
    println!("{}", value_in_cents(qrtala));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
