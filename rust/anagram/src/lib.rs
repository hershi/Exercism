use std::collections::HashMap;

pub fn anagrams_for(word : &str, inputs : &[&str]) -> Vec<String> {
    let mut result : Vec<String> = Vec::new();

    for s in inputs {
        if is_anagram(word, s) {
            result.push(s.to_string());
        }
    }

    result
}

fn is_anagram(lhs : &str, rhs: &str) -> bool {
    let lhs = lhs.to_lowercase();
    let rhs = rhs.to_lowercase();

    if lhs == rhs {
        return false;
    }

    if lhs.len() != rhs.len() {
        return false;
    }

    let mut characters = HashMap::new();

    for c in lhs.chars() {
        let entry = characters.entry(c).or_insert(0);
        *entry += 1;
    }

    for c in rhs.chars() {
        let entry = characters.entry(c).or_insert(0);

        if *entry == 0 {
            return false;
        }

        *entry -= 1;
    }

    for (_, &count) in characters.iter() {
        if count > 0 {
            return false;
        }
    }

    return true;
}
