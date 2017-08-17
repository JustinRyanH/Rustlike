//use graphics::types::Color;
use graphics::{Context, Graphics, Transformed};

use GameController;

/// Visual Configuration Information
pub struct GameViewSettings {
    /// Position from Upper left corner
    pub position: [f64; 2],
    /// World Size in number of cells
    pub size: u32,
    /// Size of world cells
    pub cell_size: u32,
}

impl GameViewSettings {
    /// Create new instance of GameViewSettings
    pub fn new() -> GameViewSettings {
        return GameViewSettings {
            position: [16 as f64; 2],
            size: 10,
            cell_size: 48,
        };
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

    fn draw_grid<G: Graphics>(&self, _: &GameController, ctx: &Context, gfx: &mut G) {
        use graphics::{Line};

        let ref settings = self.settings;

        let cell_edge = Line::new([0.8; 4], 1.0);
        for i in 0..settings.size+1 {
            let x = settings.position[0] + i as f64  * settings.cell_size as f64;
            let y = settings.position[1] + i as f64  * settings.cell_size as f64;
            let x2 = settings.position[0] +
                (settings.size as f64 * settings.cell_size as f64);
            let y2 = settings.position[1] +
                (settings.size as f64 * settings.cell_size as f64);

            let vline = [x, settings.position[1], x, y2];
            cell_edge.draw(vline, &ctx.draw_state, ctx.transform, gfx);

            let hline = [settings.position[0], y, x2, y];
            cell_edge.draw(hline, &ctx.draw_state, ctx.transform, gfx);
        }
    }

    /// Draw Game
    pub fn draw<G: Graphics>(&self, controller: &GameController, ctx: &Context, gfx: &mut G) {
        let ref settings = self.settings;

        use graphics::{Rectangle};
        let player =  Rectangle::new([1.0; 4]);

        let ref player_entity = controller.game.player;

        player.draw([settings.position[0], settings.position[1], settings.cell_size as f64, settings.cell_size as f64],
                    &ctx.draw_state,
                    ctx.transform.trans((player_entity[0] * settings.cell_size) as f64, (player_entity[1] * settings.cell_size) as f64),
                    gfx);

        self.draw_grid(controller, ctx, gfx)
    }
}
