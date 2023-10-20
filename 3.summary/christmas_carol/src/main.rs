const ORDINALS: [&str; 12] = [
    "first", "second", "third", "fourth", "fivth", "sixth", "seventh", "eighth", "nineth", "tenth",
    "eleventh", "twelvth",
];

const CARDINALS: [&str; 12] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
    "twelve",
];

const GIFTS: [&str; 12] = [
    "partridge in a pear tree",
    "turtle doves",
    "French hens",
    "calling birds",
    "gold rings",
    "geese a-laying",
    "swans a-swimming",
    "maids a-milking",
    "ladies dancing",
    "lords a-leaping",
    "pipers piping",
    "drummers drumming",
];

fn capitalize_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first_char) => {
            let rest_of_string: String = chars.collect();
            let capitalized_first_char = first_char.to_uppercase();
            let mut result = capitalized_first_char.to_string();
            result.push_str(&rest_of_string);
            result
        }
    }
}

fn compose_carol_song_verse(song: &mut String, day: usize) {
    let enry_line = format!(
        "On the {} day of Christmas my true love sent to me",
        ORDINALS[day - 1]
    );
    song.push_str(&enry_line);

    for gift in (0..day).rev() {
        song.push_str("\n");
        let amount = CARDINALS[gift];
        if gift == 0 {
            if day == 1 {
                song.push_str("A ");
            } else {
                song.push_str("And a ");
            }
        } else {
            let capitalized_amount = capitalize_first_letter(amount);
            song.push_str(&format!("{} ", &capitalized_amount));
        };
        song.push_str(GIFTS[gift]);
        if gift != 0 {
            song.push_str(",");
        }
    }
    song.push_str("\n\n");
}

fn main() {
    println!("\n\nCarol song\n\n");

    let mut result = String::new();

    for number in 1..13 {
        compose_carol_song_verse(&mut result, number)
    }
    println!("{}", &result);
}
