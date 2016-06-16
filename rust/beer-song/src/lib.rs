const COUNT_UPPER_BOUND : u32 = 100;

fn bottles_of_beer(count : u32) -> String {
    let how_many = if count > 0 {count.to_string()} else {"No more".to_string()}; 
    let bottles = if count == 1 { "bottle" } else { "bottles" };
    format!("{} {} of beer", how_many, bottles)    
}

fn get_action(ordinal : u32) -> String {
    if ordinal == 0 { return "Go to the store and buy some more".to_string()}; 

    let word = if ordinal == 1 {"it"} else {"one"};
    format!("Take {} down and pass it around", word).to_string()
}

pub fn verse(ordinal : u32) -> String {
    let bottles = bottles_of_beer(ordinal);
    let next_count = if ordinal > 0 { ordinal - 1 } else { COUNT_UPPER_BOUND - 1 };     

    format!("{} on the wall, {}.\n\
    {action}, {} on the wall.\n",
        bottles,
        bottles.to_lowercase(),
        bottles_of_beer(next_count).to_lowercase(),
        action = get_action(ordinal))
}

pub fn sing(start : u32, end : u32) -> String {
    (end..start + 1).rev()
                    .map(|ordinal| verse(ordinal))
                    .collect::<Vec<String>>()
                    .as_slice()
                    .join("\n")
}