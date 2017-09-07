use graphics::{Context, Transformed};
use opengl_graphics::GlGraphics;

use actions::Action;
use state::Stateful;
use render::Drawable;
use render::game::GameViewSettings;
use geometry::rectangle::Rectangle;

/// Debug is for debug displays
#[derive(Copy, Clone, PartialEq, Debug, Hash)]
pub struct Debug {
    rect: Rectangle<i32>,
}

impl Stateful for Debug {
    fn next(&self, _: Action) -> Self {
        self.clone()
    }
}

impl Drawable for Debug {
    fn draw<'a>(&self, settings: &'a GameViewSettings, ctx: &Context, gfx: &mut GlGraphics) {
        use graphics::Rectangle;
        Rectangle::new([0.3; 4]).draw([
            settings.position[0],
            settings.position[1],
            ( self.rect.get_width() * settings.cell_size) as f64,
            ( self.rect.get_height() * settings.cell_size) as f64
        ],
        &ctx.draw_state,
        ctx.transform.trans(
            (self.rect.get_x() * settings.cell_size) as f64,
            (self.rect.get_y() * settings.cell_size) as f64,
        ),
        gfx)
    }
}
