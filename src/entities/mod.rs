//! Drawable Ite in the universe
use graphics::{Context, Graphics};

use render::game::GameViewSettings;
use entities::player::PlayerEntity;

pub mod player;

/// Used to identify events arguments provided by traits.
#[derive(Clone, PartialEq, Debug)]
pub enum EntityKind {
    Player(PlayerEntity),
}

pub trait Entity: Sized {
    /// Get the Kind of Entity, and used to match of type
    fn kind(self) -> EntityKind;
    /// Draws the entity to openGL
    fn draw<'a, G: Graphics>(&self, settings: &'a GameViewSettings, ctx: &Context, gfx: &mut G);
}
