#[derive(Debug)]
pub struct Creature {
    power: i32,
    toughness: i32,
}

impl Creature {
    pub fn new(power: i32, toughness: i32) -> Self {
        if toughness <= 0 {
            panic!("Toughness must be greather than 0")
        }
        Creature { power, toughness }
    }
}
