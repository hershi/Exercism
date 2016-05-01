pub fn is_leap_year(year : u32) -> bool {
    let year_modulus = (year % 400 == 0,
                        year % 100 == 0,
                        year % 4 == 0);
    match year_modulus {
        (true, _, _) => true,
        (_,true,_) => false,
        (_,_,true) => true,
        _    => false
    }
}
