use sdl2::{self, render, pixels, EventPump};

use gl;
use builder::ContextBuilder;
use error::{AppResult, AppError};


fn find_sdl_gfx_driver() -> AppResult<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" || item.name == "vulkan" {
            return Ok(index as u32);
        }
    }
    Err(AppError::WindowError(String::from("Failed top find Graphics Driver")))
}

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


        let mut canvas = window.into_canvas()
            .index(find_sdl_gfx_driver()?)
            .build()?;
        canvas.set_draw_color(pixels::Color::RGB(154, 206, 235));
        let event_pump = sdl_context.event_pump()?;

        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
        canvas.window().gl_set_context_to_current()?;

        Ok(Context {
            canvas,
            event_pump,
        })
    }

    pub fn poll_iter(&mut self) -> sdl2::event::EventPollIterator {
        self.event_pump.poll_iter()
    }

    pub fn blip(&mut self) {
        unsafe {
            gl::ClearColor(154./255., 206./255., 235./255., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        self.canvas.present()
    }
}
