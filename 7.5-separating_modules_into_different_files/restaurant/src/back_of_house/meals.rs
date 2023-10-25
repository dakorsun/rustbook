// For the majority of the cases we want enums variant to be public
// So Rust makes enums variants public as default
pub enum Appetizer {
    Soup,
    Salad,
}
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}
impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}
