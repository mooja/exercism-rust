pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    fn get_bit_at(&self, n: u8) -> bool {
        self.score & (1 << n) != 0
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        use Allergen::*;
        match allergen {
            Eggs => self.get_bit_at(0),
            Peanuts => self.get_bit_at(1),
            Shellfish => self.get_bit_at(2),
            Strawberries => self.get_bit_at(3),
            Tomatoes => self.get_bit_at(4),
            Chocolate => self.get_bit_at(5),
            Pollen => self.get_bit_at(6),
            Cats => self.get_bit_at(7),
        }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        use Allergen::*;
        vec![
            Eggs,
            Peanuts,
            Shellfish,
            Strawberries,
            Tomatoes,
            Chocolate,
            Pollen,
            Cats,
        ]
        .into_iter()
        .filter(|a| self.is_allergic_to(a))
        .collect::<Vec<Allergen>>()
    }
}