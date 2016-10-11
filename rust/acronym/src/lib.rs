#[derive(Debug, PartialEq)]
enum PreviousChar {
    WordBoundary,
    Lowercase,
    Uppercase
}

enum EvalOutcome {
    Take(char),
    Skip
}

pub fn abbreviate(phrase : &str) -> String {
    // General approach:
    // Use 'scan' to process each character in turn, while retaining state
    // based on the previous character, which will be used to decide whether
    // the current character should be included in the acronym or not:
    // 1. If the previous character was a word boundary (i.e comes after a space or hyphen)
    // 2. If previous character was lowercase, and current one is uppercase (for camel-case)
    // 
    // The 'scan' iterator emits 'Take' items for characters that should be included in
    // in the acronym, and 'Skip' items for characters that should not be included. 
    // We then filter only the 'Take' items, and compose the result from them.
    phrase
        .chars()
        .scan(
            PreviousChar::WordBoundary,
            |state, c| {
                let res = Some(
                    if should_include(state, &c) { EvalOutcome::Take(c) } else { EvalOutcome::Skip }
                );
                *state = next_state(&c);

                res
            })
        .filter_map(|outcome| {
                match outcome {
                    EvalOutcome::Take(c) => c.to_uppercase().next(),
                    EvalOutcome::Skip => None 
                }
            })
        .fold(String::new(), |mut acc, c| {acc.push(c); acc})
}

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