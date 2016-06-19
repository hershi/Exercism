use std::collections::HashSet;

pub fn is_pangram(sentence : &str) -> bool {
    sentence.to_lowercase()
            .chars()
            .filter(|c| ('a' <= *c) && (*c <= 'z'))
            .collect::<HashSet<char>>()
            .len() == 26
}