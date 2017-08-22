use graphics::{Context, Graphics, Transformed};
use render::game::GameViewSettings;

use entities::{Entity, EntityKind};

/// Entity that represents the players
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Player {
    /// Where does the player exist at in world space
    location: [i32; 2],
}

impl Entity for Player {
    fn kind(self) -> EntityKind {
        return EntityKind::Player(self);
    }

    fn draw<'a, G: Graphics>(&self, settings: &'a GameViewSettings, ctx: &Context, gfx: &mut G) {
        use graphics::Rectangle;
        Rectangle::new([1.0; 4]).draw(
            [
                settings.position[0],
                settings.position[1],
                settings.cell_size as f64,
                settings.cell_size as f64
            ],
            &ctx.draw_state,
            ctx.transform.trans(
                (self.location[0] * settings.cell_size) as f64,
                (self.location[1] * settings.cell_size) as f64
            ),
            gfx);
    }
}
