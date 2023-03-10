use std::collections::HashMap;
use std::io::{self, BufRead};
//use unidecode::unidecode;

/// Map parser for map.tsv
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
    for (lineno, line) in f.lines().enumerate() {
        let line = line.unwrap();

        // split line into parts
        let parts: Vec<String> = line.split('\t').map(|s| s.to_string()).collect();

        let without_prefix: &str = parts[0].trim_start_matches("0x");
        let codepoint: u32 = u32::from_str_radix(without_prefix, 16).unwrap();
        
        let c = parts[1].chars().next().unwrap();

        if codepoint != c as u32 {
            panic!("Unicode codepoint in first column and character in second must match: Line {}, {} != {}", lineno, codepoint, c as u32);
        }

        let ascii: Vec<String> = parts.get(2..).unwrap().to_vec();

        for a in ascii {
            // check if duplicate
            if map.contains_key(&a) {
                panic!("Duplicate entry in tsv file: Line {}, {} & {} -> {}", lineno, c, map[&a], a);
            }

            map.insert(a, c);
        }
    }

    Some(map)
}


/// Map parser for map.bincode
fn load_map_from_bincode(path: &str) -> Option<HashMap<String, char>> {
    log::debug!("Reading serialized data from {}...", path);
    
    let mut file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                log::debug!("File not found: {}", e);
                return None;
            },
            _ => {
                log::error!("Error opening file: {}", e);
                return None;
            }
        }
    };

    match bincode::deserialize_from(&mut file) {
        Ok(map) => return Some(map),
        Err(e) => {
            log::error!("Error deserializing data from file: {}", e);
            return None;
        }
    };
}

/// Store map to bincode file
pub fn store_map(map: &HashMap<String, char>, path: &str) -> Result<(), &'static str> {
    let mut file = match std::fs::File::create(path) {
        Ok(file) => file,
        Err(e) => panic!("Error creating file: {}", e),
    };

    log::debug!("Serializing data to {}...", path);

    match bincode::serialize_into(&mut file, map) {
        Ok(_) => {},
        Err(_) => return Err("Error serializing data to file"),
    };

    log::debug!("Serializing data to {}... Done!", path);

    Ok(())
}

pub fn load_map(path: &str) -> Option<HashMap<String, char>> {
    // if bincode, load from bincode
    if path.ends_with(".bincode") {
        match load_map_from_bincode(path) {
            Some(map) => return Some(map),
            None => {},
        };
    }

    // if tsv, load from tsv
    if path.ends_with(".tsv") {
        match load_map_from_tsv(path) {
            Some(map) => return Some(map),
            None => {},
        };
    }

    return None;
}

#[derive(Clone, Debug)]
pub struct StringShortener {
    map: std::collections::HashMap<String, char>,
}

impl StringShortener {
    pub fn new() -> Self {
        static MAP_FILE: &'static [u8] = include_bytes!("map.bincode");
        let map = bincode::deserialize(MAP_FILE).unwrap();
        Self { map }
    }

    pub fn shorten_by_bytes(&self, input: &str) -> String {
        shorten(input, &self.map, shortest_by_bytes)
    }

    pub fn shorten_by_chars(&self, input: &str) -> String {
        shorten(input, &self.map, shortest_by_chars)
    }
}

// test create new StringShortener
#[test]
fn test_new() {
    let _ = StringShortener::new();
}


pub fn shorten(
    input: &str,
    map: &std::collections::HashMap<String, char>,
    selector: fn(Vec<String>) -> String,
) -> String {
    // if input is empty, return empty string
    if input.is_empty() {
        return String::new();
    }

    // if input is one char, return it
    if input.chars().count() == 1 {
        return input.to_string();
    }

    // get all possible replacements
    let mut replacements = Vec::new();

    // replace first i letters
    for i in 1..=input.len() {
        // get substring
        let substring: String = input.chars().take(i).collect();
        let remainder: String = input.chars().skip(i).collect();
        // if substring is in map
        if let Some(replacement) = map.get(&substring) {
            // add to replacements
            replacements
                .push(replacement.to_string() + &shorten(&remainder, map, selector));
        }
    }

    // skip this letter
    {
        let current: String = input.chars().take(1).collect();
        let remainder: String = input.chars().skip(1).collect();
        replacements.push(current + &shorten(&remainder, map, selector));
    }

    // find shortest element in replacements list
    let shortest = selector(replacements);

    // return shortest replacement
    shortest.to_string()
}

fn shortest_by_bytes(replacements: Vec<String>) -> String {
    let shortest = replacements
        .iter()
        .min_by_key(|s| s.len())
        .unwrap()
        .to_string();

    shortest
}

fn shortest_by_chars(replacements: Vec<String>) -> String {
    let shortest = replacements
        .iter()
        .min_by_key(|s| s.chars().count())
        .unwrap()
        .to_string();

    shortest
}

pub fn shorten_by_bytes(input: &str, map: &std::collections::HashMap<String, char>) -> String {
    shorten(input, map, shortest_by_bytes)
}

pub fn shorten_by_chars(input: &str, map: &std::collections::HashMap<String, char>) -> String {
    shorten(input, map, shortest_by_chars)
}