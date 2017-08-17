//! Main Controller for the Roguelike

use Game;

use piston::input::{GenericEvent};

/// Changes world state based on input from User
pub struct GameController {
    /// Information about Game
    pub game: Game
}

impl GameController {
    /// Creates new instance of GameController
    pub fn new(game: Game) -> GameController {
        return GameController{
            game: game,
        };
    }

    /// React to External Event
    pub fn event<E: GenericEvent>(&mut self, evt: &E) {
        use piston::input::{Button, Key};
        if let Some(Button::Keyboard(key)) = evt.press_args() {
            match key {
                Key::S => self.game.move_by([0, 1]),
                Key::W => self.game.move_by([0, -1]),
                Key::A => self.game.move_by([-1, 0]),
                Key::D => self.game.move_by([1, 0]),
                _ => {},
            }
        }
    }
}
