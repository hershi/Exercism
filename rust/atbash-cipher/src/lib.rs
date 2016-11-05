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
    basic_encode(phrase, &ArithmeticEncoder{})
        .as_slice()
        .chunks(5)
        .map(|x| x.into_iter().cloned().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn decode(phrase : &str) -> String {
    basic_encode(phrase, &ArithmeticEncoder{}).into_iter().collect()
}

struct ArithmeticEncoder {}

impl Encoder for ArithmeticEncoder {
    fn encode(&self, c : &char) -> char {
        match *c {
            // Use byte literals to make arithmetic easy and avoid lots of "as..."
            x@'a'...'z' => (b'z' - x as u8 + b'a') as char,
            x@_ => x
        }
    }
}

// struct MapEncoder {
//     encoding : HashMap<char, char>, 
// }

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

