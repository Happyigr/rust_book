mod back_of_house;
mod front_of_house;

// used in back_of_house with the super statement
fn deliver_order() {}

use crate::back_of_house::Appetizer as meals;
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("White");
    meal.toast = String::from("Wheat");
    println!("I'd like a {} toast, please", meal.toast);
    let order1 = meals::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // Absolut path
    hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
