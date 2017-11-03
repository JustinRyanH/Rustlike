use sdl2::{self, render, EventPump};

use gl::{self, GlContext};
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
    gl_context: GlContext,
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


        let canvas = window.into_canvas()
            .index(find_sdl_gfx_driver()?)
            .build()?;
        let event_pump = sdl_context.event_pump()?;

        gl::raw::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
        canvas.window().gl_set_context_to_current()?;

        Ok(Context {
            gl_context: Default::default(),
            canvas,
            event_pump,
        })
    }

    pub fn poll_iter(&mut self) -> sdl2::event::EventPollIterator {
        self.event_pump.poll_iter()
    }

    pub fn blip(&mut self) {
        self.gl_context.clear();
        self.canvas.present()
    }
}
