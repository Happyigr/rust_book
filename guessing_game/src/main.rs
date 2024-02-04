use rand::Rng;
use std::{cmp::Ordering, io};

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || 100 < value {
            panic!("Guess must be between 1 and 100. Got {}", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input the numeric guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Faile to read the line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if 1 < guess || guess > 100 {
            println!("The secret number will be in range of 1 and 100");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
