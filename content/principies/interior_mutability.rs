// interior mutabilitie is a pattern in rust that allow us to mutate the data, that is immutable

fn main() {
    // because of the borrowing rules this code will not compile:
    // let x = 5;
    // let y = &mut x;
    // but with RefCell() we can change it see the other in the cargo project interior_mutability
}
