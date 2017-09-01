//! Elements that are renderable in the universe
use graphics::{Context, Graphics};

use render::game::GameViewSettings;

/// Player Entity
pub mod player;

/// Behavior all entities have
pub trait Entity: Sized {
    /// Draws the entity to openGL
    fn draw<'a, G: Graphics>(&self, settings: &'a GameViewSettings, ctx: &Context, gfx: &mut G);
}
