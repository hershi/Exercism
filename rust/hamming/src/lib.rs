pub fn hamming_distance(lhs : &str, rhs: &str) -> Result<i32, &'static str> {
    if lhs.len() != rhs.len() {
        return Err("inputs of different length");
    }
    
    Ok(lhs.chars().zip(rhs.chars())
          .fold(0, |accumulator, (lhs,rhs)| { accumulator + bool_to_int(lhs != rhs) }  ))
}

fn bool_to_int(value : bool) -> i32 {
    if value { return 1;} 0
}