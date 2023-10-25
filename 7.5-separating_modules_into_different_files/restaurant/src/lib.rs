mod back_of_house;
mod front_of_house;

use crate::{back_of_house::meals, front_of_house::hosting};

// mod customer {
//     pub fn eat_at_restaurant() {
//         // This wont work because hosting has a shortcut for a upper scope, not this exactly
//         // hosting::add_to_waitlist();
//     }
// }

pub fn eat_at_restaurant() {
    // //Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    // //Relative path
    // front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = meals::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed to see or modify the
    // seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
    let order1 = meals::Appetizer::Soup;
    let order2 = meals::Appetizer::Salad;
}
