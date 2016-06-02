fn main(){
    let x = Allergies::new(&98);
    println!("Eggs {}", x.is_allergic_to(&Allergen::Eggs));
    println!("Peanuts {}", x.is_allergic_to(&Allergen::Peanuts));
    println!("Shellfish {}", x.is_allergic_to(&Allergen::Shellfish));
    println!("Strawberries {}", x.is_allergic_to(&Allergen::Strawberries));
    println!("Tomatoes {}", x.is_allergic_to(&Allergen::Tomatoes));
    println!("Chocolate {}", x.is_allergic_to(&Allergen::Chocolate));
    println!("Pollen {}", x.is_allergic_to(&Allergen::Pollen));
    println!("Cats {}", x.is_allergic_to(&Allergen::Cats));
}

#[derive(Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats
}

pub struct Allergies(u32);

impl Allergies {
    pub fn new(val : &u32) -> Allergies {
        Allergies(*val)
    }

    pub fn is_allergic_to(&self, allergen : &Allergen) -> bool {
        (self.0 & 2u32.pow(allergen.clone() as u32)) != 0
    }
}

