#[derive(Debug)]
pub struct Player {
    life: u32,
}

impl Player {
    fn new(life: u32) -> Self {
        Player { life }
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
}
