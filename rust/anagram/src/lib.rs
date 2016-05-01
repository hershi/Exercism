use std::collections::HashMap;

pub fn anagrams_for(word : &str, inputs : &[&str]) -> Vec<String> {
    let word = word.to_lowercase();
    let word_char_map = into_character_map(&word);

    inputs.iter()
        .filter(|input| {
            let lowercase_input = input.to_lowercase();
            word != lowercase_input && word_char_map.eq(&into_character_map(&lowercase_input))
        })
        .map(|s| s.to_string())
        .collect()
}

fn into_character_map(word : &str) -> HashMap<char, i32> {
    word.chars().fold(HashMap::new(), |mut char_map, c| {
        *char_map.entry(c).or_insert(0) += 1;
        char_map
    })
}

