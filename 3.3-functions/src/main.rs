// Example function definition
// fn main() {
//     println!("Hello, world!");
//     another_function();
// }
//
// fn another_function() {
//     println!("Another function.")
// }


// Example function with argument
// fn main() {
//     another_function(5);
// }
//
// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// Example function with multiple arguments
// fn main() {
//     print_labeled_measurement(5, 'h');
// }
//
// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}")
// }

// Statements and Expressions


// Function definitions are also statements;
// the entire main fn example is a statement in itself.
// fn main() {
//     // This is a statement
//     let y = 6;
// }

// Statements do not return values.
// Next example returns an error
// fn main () {
//     let x = (let y = 6);
// }

fn main() {
    let y = 
    // This is an expression: 
    {
        let x = 3;
        // That line doesn't have semicolon, in case it have that line becomes a statement and doesn't return any value
        x + 1
    };

    println!("The value of y is: {y}");
}

