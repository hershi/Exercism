#[derive(Debug, PartialEq)]
enum PreviousChar {
    WordBoundary,
    Lowercase,
    Uppercase
}

pub fn abbreviate(phrase : &str) -> String {
    // General approach:
    // Use 'filter' to keep only the characters that should be part of the acronym. Since
    // we need to keep track of the previous character type to make the decision, we
    // capture an external state variable in the closure.
    let mut state = PreviousChar::WordBoundary;
    phrase.chars()
        .filter(
            |c| {
                let res = should_include(&state, &c);
                state = next_state(&c);
                res
            })
        .collect::<String>()
        .to_uppercase()
}

// Should the current character be included in the acronym? Yes, iff:
// 1. The previous character was a word boundary (i.e space or hyphen)
// 2. The previous character was lowercase, and current one is uppercase (for camel-case)
fn should_include(state : &PreviousChar, current_char : &char) -> bool {
    *state == PreviousChar::WordBoundary ||
    (*state == PreviousChar::Lowercase && current_char.is_uppercase())
}

fn next_state(current_char : &char) -> PreviousChar {
    match current_char {
        &c if c.is_whitespace() => PreviousChar::WordBoundary,
        &c if c == '-' => PreviousChar::WordBoundary,
        &c if c.is_uppercase() => PreviousChar::Uppercase,
        _ => PreviousChar::Lowercase,
    }
}