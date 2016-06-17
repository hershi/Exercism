use std::collections::HashMap;


fn get_scoring_table() -> HashMap<char, u32> {
    let mut scoring_table : HashMap<char, u32> = HashMap::new();
    
    for c in vec!['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T'] { scoring_table.insert(c,1); }
    for c in vec!['D', 'G'] { scoring_table.insert(c,2); }
    for c in vec!['B', 'C', 'M', 'P'] { scoring_table.insert(c,3); }
    for c in vec!['F', 'H', 'V', 'W', 'Y'] { scoring_table.insert(c,4); }
    for c in vec!['K'] { scoring_table.insert(c,5); }
    for c in vec!['J', 'X'] { scoring_table.insert(c,8); }
    for c in vec!['Q', 'Z'] { scoring_table.insert(c,10); }
    
    scoring_table
}

pub fn score(word : &str) -> u32 {
    let scoring_table = get_scoring_table();
    word.to_uppercase().chars().fold(0, |score, c| score + scoring_table.get(&c).map_or(0, |x| *x))
}