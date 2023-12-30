use super::Player;

pub struct Data {
    pub player: Player,
}

impl Data {
    pub fn new(player: Player) -> Self {
        Self { player }
    }
}
