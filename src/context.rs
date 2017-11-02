use sdl2::{self, render, pixels, EventPump};

use builder::ContextBuilder;
use error::{AppResult, AppError};

pub struct Context {
    canvas: render::WindowCanvas,
    event_pump: EventPump,
}


impl Context {
    pub fn new(builder: ContextBuilder) -> AppResult<Context> {
        let sdl_context = match sdl2::init() {
            Ok(context) => context,
            Err(s) => return Err(AppError::WindowError(s)),
        };
        let video_subsystem = match sdl_context.video() {
            Ok(video) => video,
            Err(s) => return Err(AppError::WindowError(s)),
        };

        let window = builder.build(&video_subsystem)?;


        let mut canvas = window.into_canvas().build()?;
        canvas.set_draw_color(pixels::Color::RGB(154, 206, 235));
        let event_pump = sdl_context.event_pump()?;

        Ok(Context {
            canvas,
            event_pump,
        })
    }

    pub fn poll_iter(&mut self) -> sdl2::event::EventPollIterator {
        self.event_pump.poll_iter()
    }

    pub fn blip(&mut self) {
        self.canvas.clear();
        self.canvas.present()
    }
}
