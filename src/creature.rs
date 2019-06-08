use crate::attack_target::AttackTarget;

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

    pub fn attack<T: AttackTarget>(&self, target: &mut T) {
        (*target).damage(self.power);
    }
}

impl AttackTarget for Creature {
    fn damage(&mut self, value: i32) {
        if value <= 0 {
            return;
        }
        self.toughness -= value;
    }

    fn is_dead(&self) -> bool {
        self.toughness <= 0
    }
}

#[cfg(test)]
mod test {
    use crate::attack_target::AttackTarget;
    use crate::creature::Creature;

    #[test]
    fn death_by_attack() {
        let mut creature = Creature::new(1, 1);
        creature.damage(2);
        assert!(creature.is_dead())
    }

    #[test]
    fn negative_damage_is_0() {
        let mut creature = Creature::new(1, 1);
        creature.damage(-1);
        assert_eq!(creature.toughness, 1);
    }
}
