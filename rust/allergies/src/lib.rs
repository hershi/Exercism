#[derive(Clone, Debug, PartialEq)]
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

const ALL_ALERGENS : [Allergen ; 8] = 
    [ Allergen::Eggs,
      Allergen::Peanuts,
      Allergen::Shellfish,
      Allergen::Strawberries,
      Allergen::Tomatoes,
      Allergen::Chocolate,
      Allergen::Pollen,
      Allergen::Cats ];

impl Allergies {
    pub fn new(val : u32) -> Allergies {
        Allergies(val)
    }

    pub fn is_allergic_to(&self, allergen : &Allergen) -> bool {
        (self.0 & 2u32.pow(allergen.clone() as u32)) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALL_ALERGENS
            .into_iter()
            .filter(|allergen| { self.is_allergic_to(&allergen) })
            .cloned()
            .collect()
    }
}
