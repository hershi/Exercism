pub fn abbreviate(phrase : &str) -> String {
    // General approach:
    // Evaluate all pairs of consecutive charaters, and decide whether
    // the second character should be added to the acronym:
    // 1. If it comes after a space
    // 2. If it comes after a hyphen
    // 3. If it's an uppercase letter that comes after a lowercase one
    //
    // This works for all chars except for the first one, which is always part
    // of the acronym on one hand, but is never the second char in a pair of 
    // consecutive chars on the other hand. To simplify this, we add a space at
    // the beginning of the string (which means the first char now follows a space
    // which ensures it'll be part of the acronym)
    //
    // To evaluate all pairs, we zip an iterator of the phrase with a second
    // iterator of the phrase which is shifted by one character.
    std::iter::once(' ')
        .chain(phrase.chars())
        .zip(phrase.chars())
        .filter(
            |&(c1, c2)| {
                c1.is_whitespace() ||
                c1 == '-' ||
                (!c1.is_uppercase() && c2.is_uppercase()) 
            })
        .flat_map(|(_, c2)| c2.to_uppercase().next())
        .fold(String::new(), |mut acc, c| { acc.push(c); acc })
}