#[derive(Debug, Default)]
pub struct Player {
    life: u32,
}

impl Player {
    pub fn new() -> Self {
        Player { life: 20 }
    }
}
