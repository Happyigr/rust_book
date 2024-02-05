use std::thread;
use std::time::Duration;

// implementation of the unwrap_or_else() mathod
//
// impl<T> Option<T> {
//     pub fn unwrap_or_else<F>(self, f: F) -> T
//     where
//     // there are the types of closures
//     // FnOnce - can be called once, dont haave a values to take and retutns nothing
//     // FnMut - they are mutating the value in the closure, and dont movng it outside the closure
//     // Fn - they dont mutate the captured values and dont moves this values outside the function
//         F: FnOnce() -> T
//     {
//         match self {
//             Some(x) => x,
//             None => f(),
//         }
//     }
// }

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // this is a closure it is an argument that is a funciton
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
// so is the syntax of clousures simmilar to the function syntax
// fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// let add_one_v3 = |x|             { x + 1 };
// let add_one_v4 = |x|               x + 1  ;

fn main() {
    let mut list = [
        Rectangle {
            width: 13,
            heigth: 8,
        },
        Rectangle {
            width: 10,
            heigth: 2,
        },
        Rectangle {
            width: 80,
            heigth: 9,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    // example of how the closure can takes values, borrowing, immutable borrowing, and taking
    // ownership
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // we can connect the closure with a variable in the scope of this closure
    // let only_borrows = || println!("From closure: {:?}", list);
    // let mut borrows_mutably = || list.push(7);

    // we can also give the ownersip to the closure, that can be useful in threads
    // we can do it with the move word before the arguments of closure
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // println!("Before calling closure: {:?}", list);
    // only_borrows();
    // borrows_mutably();
    // println!("After calling closure: {:?}", list);

    let example_closure = |x| x;
    let a = example_closure(2);
    // we cant now put another type in this closure, because we have used this closure with another
    // type (int)
    // let b = example_closure("SSS");
    // this is another closure in which we had typed explicit what will it return
    let expensive_clousure = |num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
