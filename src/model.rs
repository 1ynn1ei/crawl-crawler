pub enum Potion {
    OfMight,
    Unknown,
}

pub enum Scroll {
    OfTeleportation,
    OfPoison,
    Unknown,
}

pub struct Morgue {
    seed: String,
    turn: usize,
    level: usize,
    potions: Vec<Potion>,
    scrolls: Vec<Scroll>,
}

impl Morgue {
    pub fn new_test(
        seed: String,
        turn: usize,
        level: usize,
        potions: Vec<Potion>,
        scrolls: Vec<Scroll>) -> Self {
        Self { seed, turn, level, potions, scrolls }
    }
}
