use super::hosting;
use crate::back_of_house;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");

    //meal.seasonal_fruit = String::from("oranges");
    //
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
