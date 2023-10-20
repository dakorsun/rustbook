use std::io;

fn main() {
    println!("Entere the Fahrenheit temp.");

    let mut far = String::new();

    io::stdin()
        .read_line(&mut far)
        .expect("Failed to proceed your input");

    let far: f32 = match far.trim().parse() {
        Ok(num) => num,
        Err(err) => {
            println!("error: {err}");
            println!("defaults to 0");
            0.0
        }
    };

    println!("You entered {far}");

    let cels: f32 = (far - 32.0) / 9.0 * 5.0;

    println!("{far} Fahrenheit is {cels} Celsius");
}
