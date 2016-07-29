#[derive(Eq, PartialEq, Debug)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal
}

pub fn sublist<T : Eq>(lhs : &[T], rhs : &[T]) -> Comparison {
    if lhs == rhs {
        return Comparison::Equal;
    }

    if lhs.len() == 0 { return Comparison::Sublist; };
    if rhs.len() == 0 { return Comparison::Superlist; };

    let (longer, shorter, rhs_longer) = 
        if lhs.len() > rhs.len() { (lhs, rhs, false) } else { (rhs, lhs, true) };

    let found = longer.windows(shorter.len()).any(|window| window == shorter); 

    match (found, rhs_longer) {
        (false, _) => Comparison::Unequal,
        (true, true) => Comparison::Sublist,
        (true, false) => Comparison::Superlist
    }
}