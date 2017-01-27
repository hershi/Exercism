pub fn get_column_name(ordinal : u32, base : u8) -> String {
    let mut result = Vec::new();

    let mut number = ordinal;
    let mut carry = 0;

    while number > 0 {
        let current_digit = (number % (base as u32)) as i8 + carry;
        number /= base as u32;

        if current_digit <= 0 && number == 0 {
            break;
        }

        let x = match current_digit {
            digit if digit <= 0 => (base as i8 + digit, -1),
            digit@_ => (digit, 0)
        };

        result.push((b'a' + x.0 as u8 - 1) as char);
        carry = x.1;
    }

    if result.len() == 0 { 
        "a".to_string() 
    } else { 
        result.iter().rev().cloned().collect() 
    }
}

pub fn get_column_name_2(ordinal : u32, base : u8) -> String {
    let mut result = vec![1];

    for _ in 1..ordinal {
        incrementt(&mut result, base);
    }

    result.iter().map(|x| (b'a' + x - 1) as char).rev().collect()
}

fn incrementt(digits : &mut Vec<u8>, base : u8) {
    let mut carry = false;
    for digit in digits.iter_mut() {
        let (new_digit, new_carry) = increment(*digit, base);
        *digit = new_digit;
        carry = new_carry;
        if !carry { break; }
    }

    if carry { digits.push(1); }
}

fn increment(digit : u8, base : u8) -> (u8, bool) {
    ((digit % base) + 1, digit == base)
}