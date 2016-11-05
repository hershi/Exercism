use std::collections::HashMap;
use std::ascii::AsciiExt;
use std::iter::*;

trait Encoder {
    fn encode(&self, c : &char) -> char;
}

fn basic_encode(phrase : &str, encoder : &Encoder) -> Vec<char> {
    phrase
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() && c.is_ascii())
        .map(|x| encoder.encode(&x) )
        .collect()
}

pub fn encode(phrase : &str) -> String {
    let encoder = ArithmeticEncoder{};
    basic_encode(phrase, &encoder)
        .into_iter()
        .zip(once(false).chain(repeat(false).take(4).chain(once(true)).cycle()))
        .fold(String::new(), |mut acc, (c, b)| {
            if b { acc.push(' ')};
            acc.push(c); 
            acc
            })
}

pub fn decode(phrase : &str) -> String {
    let encoder = ArithmeticEncoder{};

    basic_encode(phrase, &encoder).into_iter().collect()
}

struct MapEncoder {
    encoding : HashMap<char, char>, 
}

struct ArithmeticEncoder {}
impl Encoder for ArithmeticEncoder {
    fn encode(&self, c : &char) -> char {
        if !c.is_alphabetic() {
            *c 
        } 
        else {
            let dist_from_a =  (*c as u32) - ('a' as u32);
            std::char::from_u32('z' as u32 - dist_from_a).unwrap() 
        }
    }
}

// impl MapEncoder {
//     fn new() -> MapEncoder {
//         let alphabet = "abcdefghijklmnopqrstuvwxyz";
//         let map = alphabet
//                     .chars()
//                     .zip(alphabet.chars().rev())
//                     .fold(
//                         HashMap::new(), 
//                         |mut acc, pair| {
//                             acc.insert(pair.0, pair.1);
//                             acc
//                         }
//                     );

//         MapEncoder { encoding : map}
//     }
// }

// impl Encoder for MapEncoder {
//     fn encode(&self, c : &char) -> char {
//         self.encoding.get(c).map_or(*c, |x| *x)
//     }
// }

