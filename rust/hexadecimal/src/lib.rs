pub fn hex_to_int(input : &str) -> Option<u32> {
    let mut result = 0;

    for (i, c) in input.chars().rev().enumerate() {
        match char_to_digit(&c) {
            // While the digits are valid, add them to the result
            Some(d) => result += d * 16u32.pow(i as u32),
            // Return 'None' as soon as an illegal digit is found 
            None => return None
        }
    }

    // All digits were valid, so return a proper result
    Some(result)
}

// Ideally, one would use to_digit... but then again, you could say the same about
// hex_to_int, where we could use from_str_radix, but the whole point of this exercise 
// is to not use built-in functions for conversion.
fn char_to_digit(c : &char) -> Option<u32> {
    // I opted against using 'to_lower', since it's behavior is complex (returns an
    // iterator), and what we need is quite simple and can be easily covered by adding
    // one more range (we'll need to handle '0'...'9' and 'a'...'f' ayway, so we are 
    // just adding 'A'...'F')
    match *c {
        x @ 'a'...'f' => Some((x as u32) - ('a' as u32) + 10),
        x @ 'A'...'F' => Some((x as u32) - ('A' as u32) + 10),
        x @ '0'...'9' => Some((x as u32) - ('0' as u32)),
        _ => None
    }
}