extern crate sdl2;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod error;
pub mod builder;
pub mod context;


pub fn run() -> error::AppResult<()> {
    let mut context = context::Context::new(Default::default())?;
    'running: loop {
        context.blip();
        for event in context.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
