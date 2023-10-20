// Example of passing reference instead of variable
// fn main() {
//     let s1 = String::from("hello");
//
//     let len = calculate_length(&s1);
//
//     println!("The length of '{}' is {}.", s1, len)
// }
//
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// Example of not working code
// references are immutable by default
// fn main() {
//     let s = String::from("hello");
//
//     change(&s);
// }
//
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }
// ==========================

// Mutable References
// fn main() {
//     let mut s = String::from("hello");
//
//     change(&mut s);
// }
// fn change(some_tring: &mut String) -> () {
//     some_tring.push_str(", world");
// }

// Value can't have another references if it has one mutable reference to itself
// fn main() {
//     let mut s = String::from("hello");
//
//     let r1 = &mut s;
//     let r2 = &mut s;
//
//     println!("{}, {}", r1, r2);
// }

// Valid case of few mutable references, that are used in different scopes

// fn main() {
//     let pat = expr; mut s = String::from("hello");
//     {
//         let r1 = &mut s;
//     } // r1 goes out of scope here, so we can make a new reference with no problems.
//     let r2 = &mut s;
// }

// Variable also cannot have a mutable reference while it has an immutable one
// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{} and {}", r1, r2);
//     // variables r1 and r2 will not be used after this point
//
//     let r3 = &mut s; // no problem
//     println!("{}", r3);
// }
// ============================

// Dangling References
// An try to make dangling reference which result compile-time error
// fn main() {
//     let reference_to_nothing = dangle();
// }
//
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// For purpose of previous code we can do next
// fn main() {
//     let actual_value = no_dangle();
// }
//
// fn no_dangle() -> String {
//     let s = String::from("hello");
//     s
// }
