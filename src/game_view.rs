//use graphics::types::Color;
use graphics::{Context, Graphics};

use GameController;

/// Visual Configuration Information
pub struct GameViewSettings {}

impl GameViewSettings {
    /// Create new instance of GameViewSettings
    pub fn new() -> GameViewSettings {
        return GameViewSettings {};
    }
}

/// Stores Configurable Display Information
pub struct GameView {
    /// Basic Rendering Configuration Information
    pub settings: GameViewSettings,
}

impl GameView {
    /// Create new GameView
    pub fn new(settings: GameViewSettings) -> GameView {
        return GameView {
            settings: settings,
        };
    }

    /// Draw Game
    pub fn draw<G: Graphics>(&self, _: &GameController, ctx: &Context, gfx: &mut G) {
        use graphics::{Rectangle};

        Rectangle::new([1.0; 4]).draw([24.0, 24.0, 100.0, 100.0 ], &ctx.draw_state, ctx.transform, gfx);
    }
}
