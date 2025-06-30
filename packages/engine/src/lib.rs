use game::GameState;

pub struct Engine {
    state: GameState,
}

impl Engine {
    pub fn new() -> Self {
        Self { state: GameState::new() }
    }

    pub fn tick(&mut self) {
        self.state.apply_move();
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }
}
