use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use graphics::{Context, Transformed};
use opengl_graphics::GlGraphics;

use actions::Action;
use state::Stateful;
use render::Drawable;
use render::color::BasicRGB;
use render::game::GameViewSettings;
use entities::{EntityKind, Identifiable};
use geometry::rectangle::Rectangle;

/// Debug is for debug displays
#[derive(Copy, Clone, PartialEq, Debug, Hash)]
pub struct Debug {
    rect: Rectangle<i32>,
    color: BasicRGB,
}

impl Debug {
    /// Returns a new Debug element
    pub fn new(dim: [i32; 4], color: BasicRGB) -> Debug {
        Debug {
            rect: Rectangle::new(dim),
            color: color,
        }
    }
}

impl EntityKind for Debug {}

impl Stateful for Debug {
    fn next(&self, _: Action) -> Self {
        self.clone()
    }
}

impl Drawable for Debug {
    fn draw<'a>(&self, settings: &'a GameViewSettings, ctx: &Context, gfx: &mut GlGraphics) {
        use graphics::Rectangle;
        Rectangle::new(self.color.as_float(127)).draw([
            settings.position[0],
            settings.position[1],
            ( self.rect.get_width() * settings.cell_size + settings.cell_size) as f64,
            ( self.rect.get_height() * settings.cell_size + settings.cell_size) as f64
        ],
        &ctx.draw_state,
        ctx.transform.trans(
            (self.rect.get_x() * settings.cell_size) as f64,
            (self.rect.get_y() * settings.cell_size) as f64,
        ),
        gfx)
    }
}

impl Identifiable for Debug {
    fn identify(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
