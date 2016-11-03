use std::collections::HashMap;
use std::ascii::AsciiExt;

pub fn encode(phrase : &str) -> String {
    let mapping = get_mapping();
    let mut i = 0;
    phrase
        .to_lowercase()
        .chars()
        .map(|x| mapping.get(&x).map_or(x, |y| *y) )
        .filter(|c| c.is_alphanumeric() && c.is_ascii())
        .fold(String::new(), |mut acc, c| {
            if i != 0 && i % 5 == 0 { acc.push(' ') };
            i += 1;
            acc.push(c); 
            acc
            })
}

pub fn decode(phrase : &str) -> String {
    let mapping = get_mapping();

    phrase
        .to_lowercase()
        .chars()
        .map(|x| mapping.get(&x).map_or(x, |y| *y) )
        .filter(|c| c.is_alphanumeric() && c.is_ascii())
        .fold(String::new(), |mut acc, c| {
            acc.push(c); 
            acc
            })
}

pub fn get_mapping() -> HashMap<char, char> {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    alphabet
        .chars()
        .zip(alphabet.chars().rev())
        .fold(
            HashMap::new(), 
            |mut acc, pair| {
                acc.insert(pair.0, pair.1);
                acc
            }
        )
            
}