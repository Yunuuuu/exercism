use std::mem::transmute;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Allergen {
    Eggs = 1 << 0,
    Peanuts = 1 << 1,
    Shellfish = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes = 1 << 4,
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7,
}

pub struct Allergies {
    score: u32,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen = allergen.clone() as u32;
        self.score & allergen == allergen
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        (0..=7)
            .into_iter()
            .filter_map(|i| {
                let allergen: u32 = 1 << i;
                if self.score & allergen == allergen {
                    Some(unsafe { transmute(allergen as u8) })
                } else {
                    None
                }
            })
            .collect()
    }
}
