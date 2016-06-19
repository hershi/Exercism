use std::collections::HashSet;

pub fn is_pangram(sentence : &str) -> bool {
    sentence.chars()
            .map(|c| c.to_lowercase().next().unwrap())
            .filter(|c| ('a' <= *c) && (*c <= 'z'))
            .collect::<HashSet<char>>()
            .len() == 26
}