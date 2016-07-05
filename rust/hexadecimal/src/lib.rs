pub fn hex_to_int(input : &str) -> Option<u32> {
    input.chars()
         .rev()
         // For each digit convert it to the actual numeric
         // value
         .map(|x| char_to_digit(&x))
         // Each digit's actual contribution to the result is determined
         // by its position. Least-significant digit needs to be multiplied by 16^0,
         // next one needs to be multiplied by 16^1, etc.
         // We use 'enumerate' to have the position available together with the
         // digit
         .enumerate()
         // We use 'fold' to do the accumulation, but there's a twist we need
         // to handle - if any of the digits alongs the way is invalid and cannot
         // be converted, we need to return 'None'. How do we achieve that?
         // 1. We make the accumulator an Option
         // 2. Since 'digit' is an Option, whenever processing a digit, we
         // use map() to do the transformation, which means it will return
         // None if the digit was 'None'
         // 3. If the accumulator is already None, then we don't need to
         // to do any more processing, so we gate the processing of 'digit'
         // on whether 'result' is None or not. We use and_then (as opposed
         // to 'map') to avoid double-wrapping the result in Option 
         .fold(
              Some(0), 
              |result, (i, digit)| result.and_then(
                  |result| digit.map(
                      |digit| result + digit * 16u32.pow(i as u32)
                  )
              )
         ) 
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