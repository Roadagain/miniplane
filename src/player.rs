#[derive(Debug)]
pub struct Player {
    life: u32,
}

impl Player {
    pub fn new() -> Self {
        return Player{life:20}
    }
}
