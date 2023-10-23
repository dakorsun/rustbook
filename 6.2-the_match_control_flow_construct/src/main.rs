// // Example of using match pattern for sorting incoming unknown US coins
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }
//
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             //Example of using match arme with brackets for few lines of code typically
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }
// ==========================

// // Patterns That Bind to Values
// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
//
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             //Example of using match arme with brackets for few lines of code typically
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }
// ==========================

// // Matching with Option<T>
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(expr) => Some(expr + 1),
//     }
// }
//
// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
// }
// ==========================

// // Matches are Exhaustive
// // for the case where you are not covering all the possibl case in match construct - Rust won't
// // compile
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(expr) => Some(expr + 1),
//     }
// }
// ==========================

// Catch-all Patterns and the _ Placeholder
// Example of proceeding all arms with one action but few specific
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        // Another options of doing catch-all pattern
        // _ => reroll(),
        // _ => (),
    }
}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
