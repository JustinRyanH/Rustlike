use actions::Action;
use entities::player::PlayerEntity;

/// Information about Game
#[derive(Debug, PartialEq, Eq)]
pub struct GameState {
    /// Player Position
    pub player: PlayerEntity,
}

impl GameState {
    /// Create a game instance
    pub fn new(player: PlayerEntity) -> GameState {
        GameState {
            player: player,
        }
    }

    /// Gets the next state of the game after an given action
    pub fn next(&self, action: Action) -> GameState {
        match action {
            Action::MovePlayerBy { x, y } => GameState{ player:  self.player.move_by([x, y])},
            _ => GameState{ player: self.player },
        }
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;
    use actions::Action;
    use GameState;
    use entities::player::PlayerEntity;

    #[test]
    fn noop_resolves_to_original_state() {
        let subject = GameState::new(PlayerEntity::new([0, 0]));
        assert_that(&subject.next(Action::Noop)).is_equal_to(GameState { player: PlayerEntity::new([0, 0]) });
    }

    #[test]
    fn move_player_by_changes_player_state_by_given_amount() {
        let subject = GameState::new(PlayerEntity::new([5, 5]));
        assert_that(&subject.next(Action::MovePlayerBy { x: 1, y: -1 })).is_equal_to(GameState { player: PlayerEntity::new([6, 4]) })
    }
}
