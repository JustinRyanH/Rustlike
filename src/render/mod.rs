use graphics::{Context};
use opengl_graphics::GlGraphics;

use render::game::GameViewSettings;

/// Renders Game world
pub mod game;

/// An object that can be rendered
pub trait Drawable {
    /// Draws the entity to openGL
    fn draw<'a>(&self, settings: &'a GameViewSettings, ctx: &Context, gfx: &mut GlGraphics);
}
