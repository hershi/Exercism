#[derive(Debug, PartialEq)]
enum PreviousChar {
    WordBoundary,
    Lowercase,
    Uppercase
}

pub fn abbreviate(phrase : &str) -> String {
    // General approach:
    // Use 'scan' to process each character in turn, while retaining state
    // based on the previous character, which will be used to evaluate whether
    // the current character should be included in the acronym or not:
    // 1. If the previous character was a word boundary (i.e comes after a space or hyphen)
    // 3. If it's an uppercase letter that comes after a lowercase one (for camel-case)
    // 
    // The 'scan' iterator provides the result as a boolean - by zipping that iterator
    // with the original chars iterator, we can filter for only the ones that should
    // be included and compose the result from them.
    phrase
        .chars()
        .scan(
            PreviousChar::WordBoundary,
            |state, c| {
                let included_in_acronym = 
                    *state == PreviousChar::WordBoundary ||
                    (*state == PreviousChar::Lowercase && c.is_uppercase());

                *state = match c {
                    x if x.is_whitespace() => PreviousChar::WordBoundary,
                    x if x == '-' => PreviousChar::WordBoundary,
                    x if x.is_uppercase() => PreviousChar::Uppercase,
                    _ => PreviousChar::Lowercase,
                };

                Some(included_in_acronym)
            })
        .zip(phrase.chars())
        .filter(|&(included, _)| included )
        .flat_map(|(_, c)| c.to_uppercase().next())
        .fold(String::new(), |mut acc, c| {acc.push(c); acc})
}