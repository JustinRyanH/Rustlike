use actions::Action;
use entities::EntityCollection;
use entities::player::PlayerEntity;

/// Trait can take a action and be transformed
/// a new state
pub trait Stateful {
    /// Gets the next state of the Stateful object
    fn next(&self, action: Action) -> Self;
}

/// Information about Game
#[derive(Debug, PartialEq)]
pub struct GameState {
    /// Player Position
    pub player: PlayerEntity,
    /// Collection of all the entities 
    /// that exist in the world
    pub entities: EntityCollection,
}


impl GameState {
    /// Create a game instance
    pub fn new(player: PlayerEntity) -> GameState {
        GameState {
            player: player,
            entities: EntityCollection::new(),
        }
    }


}

impl Stateful for GameState {
    fn next(&self, action: Action) -> GameState {
        match action {
            Action::MovePlayerBy { x, y } => GameState{ player:  self.player.move_by([x, y]), entities: self.entities.clone() },
            _ => GameState{ player: self.player, entities: self.entities.clone() },
        }
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;
    use actions::Action;
    use GameState;
    use state::game::Stateful;
    
    use entities::EntityCollection;
    use entities::player::PlayerEntity;

    #[test]
    fn noop_resolves_to_original_state() {
        let subject = GameState::new(PlayerEntity::new([0, 0]));
        assert_that(&subject.next(Action::Noop)).is_equal_to(GameState { player: PlayerEntity::new([0, 0]), entities: EntityCollection::new() });
    }

    #[test]
    fn move_player_by_changes_player_state_by_given_amount() {
        let subject = GameState::new(PlayerEntity::new([5, 5]));
        assert_that(&subject.next(Action::MovePlayerBy { x: 1, y: -1 })).is_equal_to(GameState { player: PlayerEntity::new([6, 4]), entities: EntityCollection::new() })
    }
}
