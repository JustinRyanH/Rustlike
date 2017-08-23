use graphics::{Context, Graphics, Transformed};
use render::game::GameViewSettings;

use entities::{Entity, EntityKind};
use geometry::vector::Vector2;

/// Entity that represents the players
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PlayerEntity {
    /// Where does the player exist at in world space
    pub location: Vector2<i32>,
}

impl PlayerEntity {
    /// Creates a fresh player entity
    pub fn new(location: [i32; 2]) -> PlayerEntity {
        PlayerEntity {
            location: Vector2::new(location),
        }
    }

    /// Returns new player entity with updated location
    pub fn move_by(self, by: [i32; 2]) -> PlayerEntity {
        PlayerEntity {
            location: self.location + Vector2::new(by),
        }
    }
}

impl Entity for PlayerEntity {
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
                (self.location.get_x() * settings.cell_size) as f64,
                (self.location.get_y() * settings.cell_size) as f64
            ),
            gfx);
    }
}
