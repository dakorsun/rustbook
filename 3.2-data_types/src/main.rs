use std::io;

fn main() {
    // ----||----
    // SCALAR Types

    // ----  ----
    // Integer Types
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // Numeric oprations

    // addition
    let sum = 5 + 10;

    // substraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    // remainder
    let remainder = 43 % 5;

    // ----  ----
    // Boolean Type
    let t = true;

    let f: bool = false; // with explicit type annotation

    // Character Type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // ----||----
    // COMPOUND Types

    // ----  ----
    // Touple Types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    // ----  ----
    // Array Types
    let array = [1, 2, 3, 4, 5];
    let first = array[0];
    let second = array[1];
    // example of specifiying arrays type
    let array_typed: [i32; 5] = [1, 2, 3, 4, 5];
    // you can also initialize an array to contain the same value for each element by specifying the initial value
    let array_initialized = [3; 5]; // an array with 5 elements that will all be set tothe value 3 initially;

    // Arrays have fixed lenght and are usefull when you want your data allocated on the stack rather than the heap
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // example code to get an array index from the user
    {
        println!("Please enter an array index.");

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");

        let element = array[index];
        println!("The value of the element at index {index} is {element}");

        //This code compiles successfully. If you run this code using cargo run and enter 0, 1, 2, 3, or 4,
        // the program will print out the corresponding value at that index in the array. If you instead enter a
        // number past the end of the array, such as 10, you’ll see output like this:
        //
        // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
        // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    }
}
