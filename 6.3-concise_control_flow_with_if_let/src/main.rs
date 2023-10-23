// //Example of program that matches just for one specific case and ignores others
// fn main() {
//     let config_max = Some(3u8);
//     match config_max {
//         Some(max) => println!("The maximum is configured to be {}", max),
//         _ => (),
//     }
// }
// // The shorter code length for the exact same behaviour
// fn main() {
//     let config_max = Some(3u8);
//     if let Some(max) = config_max {
//         println!("The maximum is configured to be {}", max);
//     }
// }

// // An example to do in a new way program for coins from prev chapter
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 9;
    // the case where we are just counting amount of all-types of our coinst but quarter
    // previous approach with match pattern
    // match coin{
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state);
    //     _ => count +=1
    // }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

