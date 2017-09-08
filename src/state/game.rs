use actions::Action;
use state::Stateful;
use entities::{Entity, EntityCollection, Identifiable};
use entities::player::Player;
use entities::debug::Debug as DebugEntity;


/// Information about Game
#[derive(Debug, PartialEq)]
pub struct GameState {
    /// Collection of all the entities
    /// that exist in the world
    pub entities: EntityCollection,
}


impl GameState {
    /// Create a game instance
    pub fn new(player: Player) -> GameState {
        GameState {
            entities: EntityCollection::new().add(Entity::Player(player)).add(Entity::Debug(DebugEntity::new([4, 4, 5, 5]))),
        }
    }

    /// Returns True is library
    pub fn contains_entity(&self, id: u64) -> bool {
        self.entities.clone().into_iter().any(|entity| entity.identify() == id)
    }
}

impl Stateful for GameState {
    fn next(&self, action: Action) -> GameState {
        GameState{ entities: self.entities.next(action) }
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;
    use actions::Action;
    use GameState;
    use state::Stateful;
    
    use entities::{Entity, EntityCollection, Identifiable};
    use entities::player::Player;

    #[test]
    fn noop_resolves_to_original_state() {
        let subject = GameState::new(Player::new([0, 0]));
        let expected = Player::new([0, 0]);

        assert_that(&subject.next(Action::Noop).contains_entity(expected.identify())).is_true();
    }

    #[test]
    fn move_player_by_changes_player_state_by_given_amount() {
        let subject = GameState::new(Player::new([5, 5]));
        let expected = Player::new([6, 4]);

        assert_that(&subject.next(Action::MovePlayerBy { x: 1, y: -1 }).contains_entity(expected.identify())).is_true();
    }
}
