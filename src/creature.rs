use crate::ability::AbilityTarget;
use crate::attack_target::AttackTarget;
use crate::attacker::Attacker;
use crate::permanent::Permanent;

pub trait ICreature: AttackTarget + Attacker + Permanent {}

#[derive(Debug, PartialEq)]
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

impl AttackTarget for Creature {
    fn damage(&mut self, value: i32) -> i32 {
        if value > 0 {
            self.toughness -= value;
        }
        self.power
    }

    fn is_dead(&self) -> bool {
        self.toughness <= 0
    }
}

impl Attacker for Creature {
    fn attack<T: AttackTarget>(&mut self, target: &mut T) {
        let damage = (*target).damage(self.power);
        self.damage(damage);
    }
}

impl AbilityTarget for Creature {}

impl Permanent for Creature {
    fn destroy(&mut self) {
        if self.is_dead() {
            return;
        }
        self.toughness = 0;
    }
}

#[cfg(test)]
mod test {
    use crate::attack_target::AttackTarget;
    use crate::creature::Creature;
    use crate::permanent::Permanent;

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

    #[test]
    fn destroy() {
        let mut creature = Creature::new(1, 1);
        creature.destroy();
        assert!(creature.is_dead())
    }
}
