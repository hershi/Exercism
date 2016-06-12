pub fn reply( sentence : &str) -> &'static str {
    if sentence.is_empty() { return "Fine. Be that way!"; }
    if sentence.to_uppercase() == sentence { return "Whoa, chill out!"; }
    if sentence.ends_with("?") { return "Sure."; }
    
    "Whatever."
}