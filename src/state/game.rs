/// Information about Game
pub struct GameState {
    /// Player Position
    pub player: [i32; 2],
}

impl GameState {
    /// Create a game instance
    pub fn new() -> GameState {
        GameState {
            player: [0, 5],
        }
    }

    /// Move Player
    pub fn move_by(&mut self, by: [i32; 2]) {
        self.player = [self.player[0] + by[0], self.player[1] + by[1]]
    }
}
