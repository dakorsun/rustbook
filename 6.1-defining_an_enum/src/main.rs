// Enum Values
// // Example of defining IP addresses version types as enum
// enum IpAddrKind {
//     V4,
//     V6,
// }
//
// // Example of enum that describes a type of each self values
// enum IpAddrTyped {
//     V4(String),
//     V6(String),
// }
//
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
//
// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//
//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };
//
//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
//
//     let homeTyped = IpAddrTyped::V4(String::from("127.0.0.1"));
//
//     let loopbackTyped = IpAddrTyped::V6(String::from("::1"));
// }
// ==========================

// // Example of enum that has a variety of types emebeedded in its variants
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// // Example of another approach of caring all types of message separately, that restricts as from most helping features of enums
// struct QuitMessage;
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String);
// struct ChangeColorMessage(i32, i32, i32);
//
// // Example of enum capacity to do own implementations(methods) as same as structs
// impl Message {
//     fn call(&self) {
//         // method body would be defined here
//     }
// }
// fn main() {
//     let m = Message::Write(String::from("hello"));
//     m.call()
// }
// ==========================

// The Option Enum and Its Advantages Over Null Values
// Examples for usage of Option<T> and its None and Some(T)
fn main() {
    let some_number = Some(5);
    let some_cahr = Some("e");

    let absent_number: Option<i32> = None;

    // Example of invalid Option asessment
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    //
    // let sum = x + y; // that is intuitive way - but Rust will always force us to check validiti
    // // of our option generic type value before use it anywhere
}
