/// State that represents the game
pub mod game;

use actions::Action;

/// Trait can take a action and be transformed
/// a new state
pub trait Stateful {
    /// Gets the next state of the Stateful object
    fn next(&self, action: Action) -> Self;
}
