use std::collections::HashMap;

pub fn anagrams_for(word : &str, inputs : &[&str]) -> Vec<String> {
    let mut result : Vec<String> = Vec::new();

    let word = word.to_lowercase();
    let word_char_map = into_character_map(&word);

    for s in inputs {
        if word == s.to_lowercase() {
            continue;
        }

        if is_anagram(&word_char_map, &s) {
            result.push(s.to_string());
        }
    }

    result
}

fn into_character_map(word : &str) -> HashMap<char, i32> {
    let mut characters = HashMap::new();

    for c in word.chars() {
        let entry = characters.entry(c).or_insert(0);
        *entry += 1;
    }

    characters
}

fn is_anagram(characters : &HashMap<char, i32>, rhs: &str) -> bool {
    let rhs_chars = into_character_map(&(rhs.to_lowercase()));

    if rhs_chars.len() != characters.len() {
        return false;
    }

    for (c, count) in rhs_chars {
        match characters.get(&c) {
            Some(&x) => {if x != count {return false}},
            None => {return false}
        }
    }

    return true;
}
