use game::board::GameState;

pub struct Engine {
    state: GameState,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            state: GameState::new(),
        }
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }
}
