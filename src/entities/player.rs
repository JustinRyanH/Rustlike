use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use graphics::{Context, Transformed};
use opengl_graphics::GlGraphics;
use render::game::GameViewSettings;

use actions::Action;
use entities::{EntityKind, Identifiable};
use geometry::vector::Vector2;
use state::Stateful;
use render::Drawable;

/// Entity that represents the players
#[derive(Copy, Clone, PartialEq, Debug, Hash)]
pub struct Player {
    /// Where does the player exist at in world space
    pub location: Vector2<i32>,
}

impl Player {
    /// Creates a fresh player entity
    pub fn new(location: [i32; 2]) -> Player {
        Player {
            location: Vector2::new(location),
        }
    }

    /// Returns new player entity with updated location
    pub fn move_by(self, by: [i32; 2]) -> Player {
        Player {
            location: self.location + Vector2::new(by),
        }
    }
}

impl EntityKind for Player {}

impl Stateful for Player {
    fn next(&self, action: Action) -> Self {
        match action {
            Action::MovePlayerBy { x, y } => self.move_by([x, y]),
            _ => self.clone()
        }
    }
}

impl Drawable for Player {
    fn draw<'a>(&self, settings: &'a GameViewSettings, ctx: &Context, gfx: &mut GlGraphics) {
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

impl Identifiable for Player {
    fn identify(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
