//! Main Controller for the Roguelike

use actions::Action;
use state::game::GameState;

use piston::input::{GenericEvent};

/// Changes world state based on input from User
pub struct GameController {
    /// Information about Game
    pub game_state: GameState
}

impl GameController {
    /// Creates new instance of GameController
    pub fn new(game_state: GameState) -> GameController {
        return GameController{
            game_state: game_state,
        };
    }

    /// React to External Event
    pub fn event<E: GenericEvent>(&mut self, evt: &E) {
        use piston::input::{Button, Key};
        if let Some(Button::Keyboard(key)) = evt.press_args() {
            match key {
                Key::S => self.game_state = self.game_state.next(Action::MovePlayerBy { x: 0, y: 1 }),
                Key::W => self.game_state = self.game_state.next(Action::MovePlayerBy { x: 0, y: -1 }),
                Key::A => self.game_state = self.game_state.next(Action::MovePlayerBy { x: -1, y: 0 }),
                Key::D => self.game_state = self.game_state.next(Action::MovePlayerBy { x: 1, y: 0 }),
                _ => {},
            }
        }
    }
}
