#![deny(missing_docs)]
//! Rust based Roguelike

/// Dev Dependencies
#[cfg(test)]
extern crate spectral;

/// Regular Dependencies
extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate cgmath;

mod state;
mod render;
mod controllers;
mod actions;

use piston::window::{WindowSettings};
use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::{RenderEvent};
use glutin_window::{GlutinWindow};
use opengl_graphics::{OpenGL, GlGraphics};

pub use state::game::{GameState};
pub use controllers::game::{GameController};
pub use render::game::{GameView, GameViewSettings};


fn main() {
    let gl_version = OpenGL::V3_2;
    let settings = WindowSettings::new("Rustlike", [512; 2])
        .opengl(gl_version)
        .exit_on_esc(true);

    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window");

    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl_gfx = GlGraphics::new(gl_version);

    let state = GameState::new([0, 0]);
    let mut controller = GameController::new(state);
    let game_view_settings = GameViewSettings::new();
    let game_render = GameView::new(game_view_settings);


    while let Some(evt) = events.next(&mut window) {
        controller.event(&evt);
        if let Some(args) = evt.render_args() {
           gl_gfx.draw(args.viewport(), |ctx, gfx| {
               use graphics::{clear};

               clear([0.2; 4], gfx);
               game_render.draw(&controller, &ctx, gfx);
           });
        }
    }
}
