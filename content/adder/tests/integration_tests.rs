use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    // this compairs two values and returns the values and false if the values were not equal
    // and true if the values were equal
    assert_eq!(4, adder::add_two(2));
}
