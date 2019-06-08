#[derive(Debug)]
pub struct Player {
    life: i32,
}

impl Player {
    fn new(life: i32) -> Self {
        Player { life }
    }

    pub fn damage(&mut self, damage: i32) {
        if damage < 0 {
            return;
        }
        self.life -= damage;
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new(20)
    }
}

#[cfg(test)]
mod test {
    use super::Player;

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
}
