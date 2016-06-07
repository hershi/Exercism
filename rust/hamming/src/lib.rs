pub fn hamming_distance(lhs : &str, rhs: &str) -> Result<usize, &'static str> {
    if lhs.len() != rhs.len() {
        return Err("inputs of different length");
    }
    
    Ok(lhs.chars().zip(rhs.chars())
       .filter(|&(l,r)| l!= r)
       .count())
}
