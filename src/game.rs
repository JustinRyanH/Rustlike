/// Information about Game
pub struct Game {
    /// Player Position
    pub player: [i32; 2],
}

impl Game {
    /// Create a game instance
    pub fn new() -> Game {
        Game {
            player: [0, 5],
        }
    }

    /// Move Player
    pub fn move_by(&mut self, by: [i32; 2]) {
        self.player = [self.player[0] + by[0], self.player[1] + by[1]]
    }
}
