use crate::attack_target::AttackTarget;
use crate::creature::Creature;

#[derive(Debug)]
pub struct Player {
    life: i32,
    creatures: Vec<Creature>,
}

impl Player {
    pub fn new(life: i32, creatures: Vec<Creature>) -> Self {
        Player { life, creatures }
    }

    fn block(&mut self, damage: i32) -> i32 {
        if self.creatures.is_empty() {
            panic!("Creatures are empty");
        }
        self.creatures[0].damage(damage)
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new(20, vec![])
    }
}

impl AttackTarget for Player {
    fn damage(&mut self, damage: i32) -> i32 {
        if !self.creatures.is_empty() {
            return self.block(damage);
        }
        if damage > 0 {
            self.life -= damage;
        }
        0
    }

    fn is_dead(&self) -> bool {
        self.life <= 0
    }
}

#[cfg(test)]
mod test {
    use crate::attack_target::AttackTarget;
    use crate::attackable::Attackable;
    use crate::creature::Creature;
    use crate::player::Player;

    #[test]
    fn default_life_is_20() {
        let player: Player = Default::default();
        assert_eq!(player.life, 20)
    }

    #[test]
    fn life_can_be_negative() {
        let mut player = Player::new(1, vec![]);
        player.damage(3);
        assert_eq!(player.life, -2);
    }

    #[test]
    fn negative_damage_is_0() {
        let mut player: Player = Default::default();
        player.damage(-4);
        assert_eq!(player.life, 20);
    }

    #[test]
    fn attack_to_player() {
        let mut creature = Creature::new(2, 2);
        let mut player: Player = Default::default();
        creature.attack(&mut player);
        assert_eq!(player.life, 18);
    }

    #[test]
    fn block_creature_attack() {
        let mut player = Player::new(20, vec![Creature::new(1, 3)]);
        let mut creature = Creature::new(2, 2);
        creature.attack(&mut player);
        assert_eq!(player.life, 20);
        assert_eq!(player.creatures[0], Creature::new(1, 1));
        assert_eq!(creature, Creature::new(2, 1));
    }
}
