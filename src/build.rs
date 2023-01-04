use std::collections::HashMap;
use std::io::{self, BufRead};
use std::env;

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

fn main() {
    println!("cargo:rerun-if-changed=map.tsv");
    let map = load_map("map.tsv").unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    store_map(&map, &format!("{}/map.bincode", out_dir)).unwrap();
}