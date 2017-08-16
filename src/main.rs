extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

use piston::window::{WindowSettings};
use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::{RenderEvent};
use glutin_window::{GlutinWindow};
use opengl_graphics::{OpenGL, GlGraphics};

fn main() {
    let glVersion = OpenGL::V3_2;
    let settings = WindowSettings::new("Rustlike", [512; 2])
        .opengl(glVersion)
        .exit_on_esc(true);

    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window");

    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut glGfx = GlGraphics::new(glVersion);

    while let Some(evt) = events.next(&mut window) {
        if let Some(args) = evt.render_args() {
           glGfx.draw(args.viewport(), |_, gfx| {
               use graphics::{clear};

               clear([0.2; 4], gfx);
           });
        }
    }
}
