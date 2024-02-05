// there are two types of the tests:
// unit-tests and integration-tests
// unit-tests tests the each function in the code
// integration-tests tests the public api, so how the other programms will use our code
// example in integration_tests.rs file

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value <= 1 {
            panic!("Guess should be greater or equal to 1, was {}", value)
        } else if 100 <= value {
            panic!("Guess should be less or equal to 100, was {}", value);
        }

        Guess { value }
    }
}

pub fn greeting(name: &str) -> String {
    // String::from("Hello")
    format!("Hello {}!", name)
}
pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iternal() {
        assert_eq!(4, internal_adder(2, 2));
    }

    #[test]
    // we can also ignore some tests, and then execute them with attr -- --ignored with cargo test
    #[ignore]
    // we can't use #[should_panic] in tests with Results as a returned value
    // we can also return Result as a result of the test
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two not equals four"))
        }
    }
    #[test]
    // this type of the test will be correct if the function has panicked
    // we can write an expected attribute, that will check if the message of the error has such a
    // substring
    #[should_panic(expected = "less or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn contains_name() {
        let result = greeting("Momo");
        // we can create our custom error message
        assert!(
            result.contains("Momo"),
            "Greeting hasn\'t contains name, value: {}",
            result
        );
    }

    #[test]
    fn it_changes_the_value() {
        // this compairs two values and returns the values and false if the values were equal
        // and true if the values were not equal
        assert_ne!(2, add_two(2));
    }
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            heigth: 9,
        };
        let smaller = Rectangle {
            width: 3,
            heigth: 2,
        };
        // assert gets the bool variable and returns the bool as the result of the test
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_bigger() {
        let bigger = Rectangle {
            width: 8,
            heigth: 9,
        };
        let smaller = Rectangle {
            width: 3,
            heigth: 2,
        };
        assert!(!smaller.can_hold(&bigger));
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.heigth > other.heigth
    }
}
