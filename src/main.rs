use unic::ucd::{Block, BlockIter, GeneralCategory};
use unidecode::unidecode;
use std::collections::HashMap;
use kdam::{tqdm, BarExt};

// there exists PropList under unic::ucd::prop_list, but it is not public???


fn build_unicode_to_ascii_map() -> HashMap<String, char> {
    // build backwards map of unicode to ascii
    let mut map: HashMap<String, char> = std::collections::HashMap::new();

    // for each unicode block in first 20
    let mut pb = tqdm!(desc = "Unicode Block", position = 0);
    for block in BlockIter::new() {

        pb.update(1);
        //pb.write(format!("Now loading block {}...", block.name));

        // for each codepoint in block
        for c in tqdm!(block.range.iter(), position = 1, desc = "Codepoint") {
            if GeneralCategory::of(c) == GeneralCategory::OtherLetter
                || GeneralCategory::of(c) == GeneralCategory::OtherSymbol
                || GeneralCategory::of(c) == GeneralCategory::PrivateUse {
                continue;
            }

            // skip if ideographic
            //if PropList::of(c).contains(PropList::Ideographic) {
            //    continue;
            //}

            let ascii = unidecode(&c.to_string());

            if !ascii.is_empty() && ascii.len() > 1 {
                if map.contains_key(&ascii) {
                    if map[&ascii].to_string().chars().count() > c.to_string().chars().count() {
                        map.insert(ascii, c);
                    } else {
                        continue;
                    }
                } else {
                    map.insert(ascii, c);
                }
            }
        }
    }
    eprint!("{}", "\n".repeat(2));
    map
}

fn replace_all(input: &str, map: &std::collections::HashMap<String, char>) -> String {
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
            replacements.push(
                replacement.to_string() + &replace_all(&input[i..], map),
            );
        }
    }

    // skip this letter
    replacements.push(input[..1].to_string() + &replace_all(&input[1..], map));

    // find shortest element in replacements list 
    let shortest = replacements.iter().min_by_key(|x| x.chars().count()).unwrap();

    // return shortest replacement
    shortest.to_string()
}

fn main() {
    let map = build_unicode_to_ascii_map();
    println!("Map size: {}", map.len());

    // take string input, ascii
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    println!("{} ({})", input, input.chars().count());

    // replace all
    let replaced = replace_all(&input, &map);
    // output
    println!("{} ({})", replaced, replaced.chars().count());
}
