pub struct Roman {
    string : String,
}

impl Roman {
    pub fn from(mut number : u32) -> Roman {
        let map = [
            (1,"I"),
            (4,"IV"),
            (5,"V"),
            (9,"IX"),
            (10,"X"),
            (40,"XL"),
            (50,"L"),
            (90,"XC"),
            (100,"C"),
            (400,"CD"),
            (500,"D"),
            (900,"CM"),
            (1000,"M")
            ];

        let mut result = Roman { string : String::new() };
        while number > 0 {
            let item = map.iter().filter(|c| c.0 <= number).last().unwrap();
            result.string.push_str(item.1);
            number -= item.0;
        }

        result
    }

    pub fn to_string(&self) -> &String {
        &self.string
    }
}
