#[allow(unused_variables)]
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, ()> {
    let invalid_bases = [0,1];
    let bases = [from_base, to_base];

    if  bases.iter().any(|base| invalid_bases.iter().any(|x| base == x)) || 
        number.iter().any(|&digit| digit >= from_base) {
        return Err(());
    }

    let mut number = number.iter().fold(0, |acc, digit| acc * from_base + digit );

    let mut res = Vec::new();
    while number > 0 {
        res.push(number % to_base);
        number = number / to_base;
    }

    Ok(res.into_iter().rev().collect::<Vec<u32>>())
}
