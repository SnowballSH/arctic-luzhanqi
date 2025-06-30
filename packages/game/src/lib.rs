use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameState {
    pub turn: u32,
}

impl GameState {
    pub fn new() -> Self {
        Self { turn: 0 }
    }

    pub fn apply_move(&mut self) {
        self.turn += 1;
    }
}
