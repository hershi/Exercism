pub fn hex_to_int(input : &str) -> Option<u32> {
    hex_to_int_error(input).ok()
}

fn hex_to_int_error(input : &str) -> Result<u32, &'static str> {
    let mut result = 0;

    for (i, c) in input.chars().rev().enumerate() {
        let digit = try!(char_to_digit(&c));
        result += digit * 16u32.pow(i as u32);
    }

    Ok(result)
}

// Ideally, one would use to_digit... but then again, you could say the same about
// hex_to_int, where we could use from_str_radix, but the whole point of this exercise 
// is to not use built-in functions for conversion.
fn char_to_digit(c : &char) -> Result<u32, &'static str> {
    // I opted against using 'to_lower', since it's behavior is complex (returns an
    // iterator), and what we need is quite simple and can be easily covered by adding
    // one more range (we'll need to handle '0'...'9' and 'a'...'f' ayway, so we are 
    // just adding 'A'...'F')
    match *c {
        x @ 'a'...'f' => Ok((x as u32) - ('a' as u32) + 10),
        x @ 'A'...'F' => Ok((x as u32) - ('A' as u32) + 10),
        x @ '0'...'9' => Ok((x as u32) - ('0' as u32)),
        _ => Err("Failed parsing")
    }
}