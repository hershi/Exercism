use std::collections::HashSet;

pub fn is_pangram(sentence : &str) -> bool {
    sentence.chars()
            .filter(|c| {let lc = c.to_lowercase().next().unwrap(); ('a' <= lc) && (lc <= 'z')})
            .collect::<HashSet<char>>()
            .len() == 26
}