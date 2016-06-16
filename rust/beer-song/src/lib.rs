const COUNT_UPPER_BOUND : u32 = 100;

fn how_many(count : u32) -> String {
    if count > 0 {count.to_string()} else {"No more".to_string()}
}

fn bottle_or_bottles(count : u32) -> String {
    if count == 1 { "bottle".to_string() } else { "bottles".to_string() }
}

fn get_next_count(count : u32) -> u32 {
    if count > 0 { count - 1 } else { COUNT_UPPER_BOUND - 1 }     
}

fn get_action(ordinal : u32) -> String {
    if ordinal == 0 {"Go to the store and buy some more".to_string()} 
    else {format!("Take {} down and pass it around", 
                  if ordinal == 1 {"it"} else {"one"})}.to_string()
}

fn verse_internal(ordinal : u32) -> String {
    let next_ordinal = get_next_count(ordinal);
    let count = how_many(ordinal);


    format!("{} {bottles} of beer on the wall, {} {bottles} of beer.\n\
    {action}, {next_count} {next_bottles} of beer on the wall.\n\n",
        count,
        count.to_lowercase(),
        bottles = bottle_or_bottles(ordinal),  
        action = get_action(ordinal),    
        next_count = how_many(next_ordinal).to_lowercase(),
        next_bottles = bottle_or_bottles(next_ordinal)
    )
}

pub fn verse(ordinal : u32) -> String {
    let mut res = verse_internal(ordinal);
    res.pop();
    res
}

pub fn sing(start : u32, end : u32) -> String {
    let mut res = (0..start - end + 1).fold(
        String::new(), 
        |mut acc, i| {acc.push_str(&verse_internal(start - i)); acc});
    res.pop();
    res
}