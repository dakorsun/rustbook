// Example of common programming problem
// Given a string of words from 1 to n, separated by spaces
// return first word from given string
fn main() {
    let mut s = String::from("test words here");
    // let result_index = first_word(&s); // result_index will get the value 4
    // println!("result_index - {}", result_index);
    let word = first_word(&s);
    println!("result word - {}", word);
    s.clear() // this empties the String, making it equal to ""
              // result_index still has the value 4 here, but there's no more string that
              // we could meaningfully use the value 4 with. result_index now is totally invalid!

    // Asking immutable reference for 's' through ref 'word' will fail at compile time
    // 's' by this moment has mutable reference by calling s.clear()
    // println!("result word - {}", word);
}

// The way to solve problem by returning index of first space character
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//
//     s.len()
// }

// The way to solve problem by returning slice of source String to indicate desired string part
// fn first_word(s: &String) -> &str {
// Write of this signature allow us to use function for both &String and &str
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// ============================

// String Literals as Slices 
// now its easy to realise that string literals are just a slices pointing to specific point of the binary.
// That's also why STRING LITERALS ARE IMMUTABLE, &str is an immutable reference



