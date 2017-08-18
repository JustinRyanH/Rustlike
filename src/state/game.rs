use actions::Action;

/// Information about Game
#[derive(Debug, PartialEq, Eq)]
pub struct GameState {
    /// Player Position
    pub player: [i32; 2],
}

impl GameState {
    /// Create a game instance
    pub fn new(player: [i32; 2]) -> GameState {
        GameState {
            player: player,
        }
    }

    /// Gets the next state of the game after an given action
    pub fn next(&self, action: Action) -> GameState {
        match action {
            Action::MovePlayerBy { x, y } => GameState{ player: [ self.player[0] + x, self.player[1] + y] },
            _ => GameState{ player: self.player },
        }
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;
    use actions::Action;
    use GameState;

    #[test]
    fn noop_resolves_to_original_state() {
        let subject = GameState::new([0, 0]);
        assert_that(&subject.next(Action::Noop)).is_equal_to(GameState { player: [0, 0] });
    }

    #[test]
    fn move_player_by_changes_player_state_by_given_amount() {
        let subject = GameState::new([5, 5]);
        assert_that(&subject.next(Action::MovePlayerBy { x: 1, y: -1 })).is_equal_to(GameState { player: [6, 4]})
    }
}
