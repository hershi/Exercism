extern crate regex;

use std::collections::HashMap;
use regex::Regex;

pub fn word_count(s: &str) -> HashMap<String, u32> {
    let word_splitter = Regex::new(r"[^a-z0-9]+").unwrap();
    word_splitter.split(&s.to_lowercase()).filter(|word| !word.is_empty()).fold(HashMap::new(), 
        |mut word_map, word| { 
            *(word_map.entry(String::from(word)).or_insert(0)) += 1;
            word_map
        })
}