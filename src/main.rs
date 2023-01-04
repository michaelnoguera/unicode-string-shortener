
use unishorten::*;

use kdam::{tqdm, BarExt};
use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use unic_char_range::{CharRange};
use unidecode::unidecode;

fn main() {
    let map: HashMap<String, char>;

    // if there is a map.bincode file, use that
    map = match load_map_from_bincode("map.bincode") {
        Some(map) => map,
        None => {
            // if there is no map.bincode file, load from map.tsv
            let map = match load_map_from_tsv("map.tsv") {
                Some(map) => map,
                None => panic!("Unable to load map from tsv file"),
            };

            // once loaded, store map as bincode
            store_map_as_bincode(&map, "map.bincode").expect("Error storing map as bincode");

            map
        }
    };

    println!("Map size: {}", map.len());

        

    // take string input, ascii
    let mut input = String::new();

    // prompt user for input and read from same line
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
    let replaced_bytes = replace_all(&input, &map, choose_shortest_by_bytes);
    println!(
        "{: <30} {: <20} ({})",
        "Shortest in bytes:",
        replaced_bytes,
        replaced_bytes.len()
    );

    let replaced_chars = replace_all(&input, &map, choose_shortest_by_chars);
    println!(
        "{: <30} {: <20} ({})",
        "Shortest in characters used:",
        replaced_chars,
        replaced_chars.chars().count()
    );
}
