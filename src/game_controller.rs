//! Main Controller for the Roguelike

use piston::input::GenericEvent;

/// Changes world state based on input from User
pub struct GameController {}

impl GameController {
    /// Creates new instance of GameController
    pub fn new() -> GameController {
        return GameController{};
    }

    /// React to External Event
    pub fn event<E: GenericEvent>(&mut self, _: &E) {

    }
}
