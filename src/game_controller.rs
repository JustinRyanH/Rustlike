//! Main Controller for the Roguelike

use Game;
use piston::input::GenericEvent;

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
    }
}
