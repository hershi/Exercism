extern crate regex;

use std::collections::HashMap;
use regex::Regex;

pub fn word_count(s: &str) -> HashMap<String, u32> {
    let word_splitter = Regex::new(r"[:^alnum:]+").unwrap();
    word_splitter.split(s).filter(|word| !word.is_empty()).fold(
        HashMap::new(), 
        |mut word_map, word| { 
            *(word_map.entry(word.to_lowercase()).or_insert(0)) += 1;
            word_map
        })
}