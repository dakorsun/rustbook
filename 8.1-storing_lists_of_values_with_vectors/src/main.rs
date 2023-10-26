fn main() {
    // Creating a New Vector
    // // Example of default creation of empty vector
    // let v: Vec<i32> = Vec::new();
    // // Example of creation of vector with macro and with not straight type assignmenf
    // let v = vec![1, 2, 3];
    // =================================

    // Updating a Vector
    // Vector follows the same mutability rules;
    // let mut v = Vec::new();
    //
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);
    // =================================

    // Reading Elements of Vectors
    // // Example of getting single values from vector
    // let v = vec![1,2,3,4,5];
    // // Exact index accession example
    // // It can make code to panic in case of accessing value that out of vec bounds
    // let third: &i32 = &v[2];
    // println!("The third element is {third}");
    //
    // // Accessino through get mechanisms
    // // It's success guaranteed by Optional enum
    // let thirdOptional : Option<&i32> = v.get(2);
    // match thirdOptional {
    //     Some(expr) => println!("The third optional element is {expr}");,
    //     None => println!("THere is no third element.");,
    // }

    // // Example of accession unexisting value
    // let v = vec![1, 2, 3, 4, 5];
    // // This will cause code panic
    // let does_not_exist = &v[100];
    // // This will make Option with None
    // let does_not_exist = &v.get(100);

    // // Borrowing rules works as same as for other heap stored data structures
    // // Since by updating vector - it might happen to be much big to require a new memory space to reallocate itself. That will loose all the references that might be at the moment
    // // So again a rule - it can't be mutation if it has valid references already
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("THe first element is: {first}");
    // =================================

    // // Iterating over the Values in a Vector
    // // Example of simple vector iteration
    // let v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{i}");
    // }
    //
    // // It is also possible to iterate over mutable references
    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     we are using * dereference here
    //     *i += 50;
    // }
    // =================================

    // // Using an Enum to Store Multiple Types
    // // Example of the usage for the spreadsheets cell, which can be
    // // integer/floating_point_number/string
    //
    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }
    //
    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12),
    // ];
}
