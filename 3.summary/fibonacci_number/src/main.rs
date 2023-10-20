use rand::Rng;
use std::io;

fn main() {
    println!(
        "The day has come,\nwe are going to find fibonacci number\nWhat step we are searching at?"
    );

    let mut step = String::new();

    io::stdin().read_line(&mut step).expect("bruh");

    let level: u128 = match step.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("crap: {e}");
            let random: u128 = rand::thread_rng().gen_range(1..=40);
            println!("gotta to random. Let it be - {random}");
            random
        }
    };

    let result: u128 = get_fibonacci_number(level);

    println!("result: {result}");
}

fn get_fibonacci_number(step: u128) -> u128 {
    if step == 0 {
        return 0;
    }
    if step == 1 {
        return 1;
    }
    if step == 2 {
        return 1;
    }
    return get_fibonacci_number(step - 1) + get_fibonacci_number(step - 2);
}
