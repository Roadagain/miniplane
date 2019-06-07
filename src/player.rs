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
