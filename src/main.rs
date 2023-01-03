use kdam::{tqdm, BarExt};
use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use unic_char_range::{CharRange};
use unidecode::unidecode;

#[allow(dead_code)]
fn build_unicode_to_ascii_map() -> HashMap<String, char> {
    // build backwards map of unicode to ascii
    let mut map: HashMap<String, char> = std::collections::HashMap::new();

    // write map to file
    let mut file = std::fs::File::create("map.txt").unwrap();

    //In addition, a number of Latin-like characters are encoded in the Currency Symbols, Control Pictures, CJK Compatibility, Enclosed Alphanumerics, Enclosed CJK Letters and Months, Mathematical Alphanumeric Symbols, and Enclosed Alphanumeric Supplement blocks, but, although they are Latin letters graphically, they have the script property common, and, so, do not belong to the Latin script in Unicode terms. Lisu also consists almost entirely of Latin forms, but uses its own script property.

    let ranges = vec![
        /* Latin blocks */
        (0x0080, 0x00FF), // Latin-1 Supplement
        (0x0100, 0x017F), // Latin Extended-A
        (0x0180, 0x024F), // Latin Extended-B
        (0x0250, 0x02AF), // IPA Extensions
        (0x02B0, 0x02FF), // Spacing Modifier Letters
        (0x1D00, 0x1D7F), // Phonetic Extensions
        (0x1D80, 0x1DBF), // Phonetic Extensions Supplement
        (0x1E00, 0x1EFF), // Latin Extended Additional
        (0x2070, 0x209F), // Superscripts and Subscripts
        (0x2100, 0x214F), // Letterlike Symbols
        (0x2150, 0x218F), // Number Forms
        (0x2C60, 0x2C7F), // Latin Extended-C
        (0xA720, 0xA7FF), // Latin Extended-D
        (0xAB30, 0xAB6F), // Latin Extended-E
        (0xFB00, 0xFB4F), // Alphabetic Presentation Forms (Latin ligatures)
        (0xFF00, 0xFFEF), // Halfwidth and Fullwidth Forms
        (0x0780, 0x107B), // Latin Extended-F
        (0xDF00, 0x1DFF), // Latin Extended-G
        /* Other blocks containing latin characters */
        (0x20A0, 0x20CF),   // Currency Symbols
        (0x2200, 0x22FF),   // Mathematical Operators
        (0x2400, 0x243F),   // Control Pictures
        //(0x3300, 0x33FF),   // CJK Compatibility
        (0x2460, 0x24FF),   // Enclosed Alphanumerics
        (0x1F100, 0x1F1FF), // Enclosed Alphanumeric Supplement
        //(0x3200, 0x32FF),   // Enclosed CJK Letters and Months
        (0x1F700, 0x1F77F), // Alchemical symbols
    ];

    // for each unicode block
    let mut pb = tqdm!(desc = "Range", position = 0, total = ranges.len());

    for r in ranges {
        pb.update(1);

        let a: char = match char::from_u32(r.0) {
            Some(c) => c,
            None => continue,
        };
        let b: char = match char::from_u32(r.1) {
            Some(c) => c,
            None => continue,
        };

        let r: CharRange = CharRange::closed(a, b);

        // for each codepoint in block
        for c in tqdm!(r.iter(), position = 1, desc = "Codepoint") {
            let ascii = unidecode(&c.to_string());

            if !ascii.is_empty() && ascii != "[?]" {
                file.write(format!("{:#08x}\t{}\t{}\n", c as u32, c, ascii).as_bytes())
                    .expect("Unable to write data");
                map.insert(ascii, c);
            } else {
                file.write(format!("{:#08x}\t{}\t\n", c as u32, c).as_bytes())
                    .expect("Unable to write data");
            }
        }
    }
    eprint!("{}", "\n".repeat(2));
    map
}

/* Read from map.tsv */
fn load_map_from_tsv(path: &str) -> Option<HashMap<String, char>> {
    let mut map: HashMap<String, char> = std::collections::HashMap::new();

    let f = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => return None,
            _ => {
                panic!("Error opening file: {}", e);
            }
        }
    };

    let f = std::io::BufReader::new(f);

    // read each line
    for line in f.lines() {
        let line = line.unwrap();

        // split line into parts
        let parts: Vec<String> = line.split('\t').map(|s| s.to_string()).collect();

        let without_prefix: &str = parts[0].trim_start_matches("0x");
        let codepoint: u32 = u32::from_str_radix(without_prefix, 16).unwrap();
        
        let c = parts[1].chars().next().unwrap();

        assert!(codepoint == c as u32);

        let ascii: Vec<String> = parts.get(2..).unwrap().to_vec();

        for a in ascii {
            // check if duplicate
            if map.contains_key(&a) {
                panic!("Duplicate entry in tsv file: {} & {} -> {}", c, map[&a], a);
            }

            map.insert(a, c);
        }
    }

    Some(map)
}

fn store_map_as_bincode(map: &HashMap<String, char>, path: &str) -> Result<(), &'static str> {
    let mut file = match std::fs::File::create(path) {
        Ok(file) => file,
        Err(e) => panic!("Error creating file: {}", e),
    };

    match bincode::serialize_into(&mut file, map) {
        Ok(_) => {},
        Err(_) => return Err("Error serializing data to file"),
    };

    Ok(())
}

fn load_map_from_bincode(file: &str) -> Option<HashMap<String, char>> {
    let mut file = match std::fs::File::open(file) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => return None,
            _ => {
                panic!("Error opening file: {}", e);
            }
        }
    };

    match bincode::deserialize_from(&mut file) {
        Ok(map) => return Some(map),
        Err(e) => panic!("Error deserializing data from file: {}", e)
    };
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
