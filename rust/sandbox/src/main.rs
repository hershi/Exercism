use std::collections::HashMap;

fn main(){
    println!("{:?}", into_character_map());
    println!("{:?}", bar("helloooooo"));
}

fn into_character_map() -> HashMap<char, i32> {
    let mut x = HashMap::new();
    x.insert('c',2);
    x.insert('d',3);

    x
}


fn bar(word: &str) -> HashMap<char, i32> {
    word.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    })
}
