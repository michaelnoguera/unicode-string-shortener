use kdam::{tqdm, BarExt};
use std::collections::HashMap;
use std::io::{self, Write};
use unic::ucd::{Block, BlockIter, GeneralCategory, Name};
use unic_char_range::{chars, CharRange};
use unidecode::unidecode;

// there exists PropList under unic::ucd::prop_list, but it is not public???

fn build_unicode_to_ascii_map() -> HashMap<String, char> {
    // build backwards map of unicode to ascii
    let mut map: HashMap<String, char> = std::collections::HashMap::new();

    // write map to file
    let mut file = std::fs::File::create("map.txt").unwrap();

    let ranges = vec![
        (0x0000, 0x007F), // Basic Latin
        (0x0080, 0x00FF), // Latin-1 Supplement
        (0x0100, 0x017F), // Latin Extended-A
        (0x0180, 0x024F), // Latin Extended-B
        (0x0250, 0x02AF), // IPA Extensions
    ];

    // for each unicode block
    let mut pb = tqdm!(desc = "Range", position = 0, total = ranges.len());

    for r in ranges {
        pb.update(1);

        let a: char = char::from_u32(r.0).unwrap();
        let b: char = char::from_u32(r.1).unwrap();

        let r: CharRange = CharRange::closed(a,b);

        // for each codepoint in block
        for c in tqdm!(r.iter(), position = 1, desc = "Codepoint") {
            /*if GeneralCategory::of(c) == GeneralCategory::OtherLetter
                || GeneralCategory::of(c) == GeneralCategory::PrivateUse
            {
                continue;
            } */

            // skip if ideographic
            //if PropList::of(c).contains(PropList::Ideographic) {
            //    continue;
            //}

            let ascii = unidecode(&c.to_string());

            if !ascii.is_empty() && ascii.len() > 1 {
                if map.contains_key(&ascii) {
                    if map[&ascii].to_string().chars().count() > c.to_string().chars().count() {
                        file.write(format!("U+{:#04x}: {} -> {}\n", c as u8, ascii, c).as_bytes())
                            .expect("Unable to write data");
                        map.insert(ascii, c);
                    } else {
                        continue;
                    }
                } else {
                    file.write(format!("U+{:#04x}: {} -> {}\n", c as u8, ascii, c).as_bytes())
                            .expect("Unable to write data");
                    map.insert(ascii, c);
                }
            }
        }
    }
    eprint!("{}", "\n".repeat(2));
    map
}

fn choose_shortest_by_bytes(replacements: Vec<String>) -> String {
    let shortest = replacements
        .iter()
        .min_by_key(|s| s.len())
        .unwrap()
        .to_string();

    shortest
}

fn choose_shortest_by_chars(replacements: Vec<String>) -> String {
    let shortest = replacements
        .iter()
        .min_by_key(|s| s.chars().count())
        .unwrap()
        .to_string();

    shortest
}

fn replace_all(
    input: &str,
    map: &std::collections::HashMap<String, char>,
    choose_shortest: fn(Vec<String>) -> String,
) -> String {
    // if input is empty, return empty string
    if input.is_empty() {
        return String::new();
    }

    // if input is one char, return it
    if input.len() == 1 {
        return input.to_string();
    }

    // get all possible replacements
    let mut replacements = Vec::new();

    // replace this letter
    for i in 1..=input.len() {
        // get substring
        let substring = &input[..i];
        // if substring is in map
        if let Some(replacement) = map.get(substring) {
            // add to replacements
            replacements
                .push(replacement.to_string() + &replace_all(&input[i..], map, choose_shortest));
        }
    }

    // skip this letter
    replacements.push(input[..1].to_string() + &replace_all(&input[1..], map, choose_shortest));

    // find shortest element in replacements list
    let shortest = choose_shortest(replacements);

    // return shortest replacement
    shortest.to_string()
}

fn main() {
    let map = build_unicode_to_ascii_map();
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