pub fn reply( sentence : &str) -> &str {
    let eval_nothing_said = |s : &str| s.is_empty();
    let eval_shouting = |s : &str| s.to_uppercase() == s;
    let eval_question = |s : &str| s.ends_with("?");
      
    let evaluators : Vec<(&Fn(&str)->bool, &str)> = vec![
        (&eval_nothing_said, "Fine. Be that way!" ),
        (&eval_shouting, "Whoa, chill out!" ),
        (&eval_question, "Sure.")
    ];
    
    evaluators.iter()
              .find(|element| element.0(sentence))
              .map_or("Whatever.", |element| element.1)
}