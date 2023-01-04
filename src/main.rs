
use unishorten::{MAP, shorten_by_bytes, shorten_by_chars};
use std::io::{self, Write};

fn main() {
    // print map
    println!("Map:");
    for (key, value) in MAP.iter() {
        println!("{} -> {}", key, value);
    }

    let mut input = String::new();
    print!("Enter string to shorten: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim();

    println!(
        "{: <30} {: <20} ({})",
        "Input:",
        input,
        input.chars().count()
    );

    // replace all
    let replaced_bytes = shorten_by_bytes(&input);
    println!(
        "{: <30} {: <20} ({})",
        "Shortest in bytes:",
        replaced_bytes,
        replaced_bytes.len()
    );

    let replaced_chars = shorten_by_chars(&input);
    println!(
        "{: <30} {: <20} ({})",
        "Shortest in characters used:",
        replaced_chars,
        replaced_chars.chars().count()
    );
}
