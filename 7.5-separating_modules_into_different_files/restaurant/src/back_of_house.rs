pub mod meals;

pub fn fix_incorrect_order() {
    cook_order();
    super::front_of_house::serving::serve_order();
}

fn cook_order() {}
