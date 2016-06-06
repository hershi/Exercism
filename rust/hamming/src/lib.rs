pub fn hamming_distance(lhs : &str, rhs: &str) -> Result<i32, &'static str> {
    if lhs.len() != rhs.len() {
        return Err("inputs of different length");
    }
    
    Ok(lhs.chars().zip(rhs.chars())
       .fold(0, |accumulator, (lhs,rhs)| { accumulator + (if lhs != rhs {1} else {0} )} )
    )
}
