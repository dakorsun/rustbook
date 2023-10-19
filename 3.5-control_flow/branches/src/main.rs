// Common example with if..else block
// fn main() {
//     let number = 3;
//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false")
//     }
// }

// Not working example - Rust doesn't try to convert non-Boolean types to Boolean
// fn main() {
//     let number = 3;
//
//     if number {
//         println!("number was three");
//     }
// }

// Example of working alternative for auto converting non-Booleans
// fn main() {
//     let number = 3;
//
//     if number != 0 {
//         println!("number was something other than zero");
//     }
// }

// Example with multiple if..else condition
// fn main() {
//     let number = 6;
//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3 or 2");
//     }
// }

// If is an expression, so we can use it on the right side of a let statement
// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };
//
//     println!("the value of number is: {number}");
// }

// Pay attention for if..else arms types mismatch of return expression
// Not working example
fn main() {
    let condition = true;
    let number = if condition { 5 } else { "six" };

    println!("the value of number is: {number}");
}
