
use unishorten::{shorten_by_bytes, shorten_by_chars, load_map, store_map};
use std::io::{self, Write};
use clap::{arg, Command};

fn main() {
    let matches = Command::new("unishorten")
        .version("0.1.0")
        .author("Michael Noguera")
        .about("Shortens ascii strings by substituting unicode characters that look like more than one ascii character")
        .arg(arg!([input] "string to shorten"))
        .arg(arg!(-i --interactive "interactive mode"))
        .get_matches();

    let mut input = String::new();
    if matches.get_flag("interactive") {
        print!("Enter string to shorten: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
    } else {
        input = match matches.get_one::<String>("input") {
            Some(c) => c.to_string(),
            None => {
                println!("No input provided");
                return;
            }
        }
    }

    let map = load_map("src/map.tsv").unwrap();
    store_map(&map, "src/map.bincode").unwrap();
    
    println!(
        "{: <30} {: <20} ({})",
        "Input:",
        input,
        input.chars().count()
    );

    // replace all
    let replaced_bytes = shorten_by_bytes(&input, &map);
    println!(
        "{: <30} {: <20} ({})",
        "Shortest in bytes:",
        replaced_bytes,
        replaced_bytes.len()
    );

    let replaced_chars = shorten_by_chars(&input, &map);
    println!(
        "{: <30} {: <20} ({})",
        "Shortest in characters used:",
        replaced_chars,
        replaced_chars.chars().count()
    );
}
