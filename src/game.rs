/// Information about Game
pub struct Game {
    /// Player Position
    pub player: [u32; 2],
}

impl Game {
    /// Create a game instance
    pub fn new() -> Game {
        Game {
            player: [0, 5]
        }
    }
}
