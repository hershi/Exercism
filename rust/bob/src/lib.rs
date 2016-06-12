pub fn reply( sentence : &str) -> &'static str {
    let evaluators : Vec<(Box<Fn(&str)->bool>, &str)> = vec![
        (Box::new(|s : &str| s.is_empty()), "Fine. Be that way!" ),
        (Box::new(|s : &str| s.to_uppercase() == s), "Whoa, chill out!" ),
        (Box::new(|s : &str| s.ends_with("?")), "Sure.")
    ];
    
    evaluators.iter()
              .find(|element| element.0(sentence))
              .map_or("Whatever.", |element| element.1)
}