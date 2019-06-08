use crate::attackable::Attackable;

#[derive(Debug)]
pub struct Player {
    life: i32,
}

impl Player {
    fn new(life: i32) -> Self {
        Player { life }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new(20)
    }
}

impl Attackable for Player {
    fn damage(&mut self, damage: i32) {
        if damage < 0 {
            return;
        }
        self.life -= damage;
    }

    fn is_dead(&self) -> bool {
        self.life <= 0
    }
}

#[cfg(test)]
mod test {
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
        let mut player = Player::new(1);
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
        let creature = Creature::new(2, 2);
        let mut player: Player = Default::default();
        creature.attack(&mut player);
        assert_eq!(player.life, 18);
    }
}
