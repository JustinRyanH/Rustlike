use sdl2::{self, render, EventPump};
use sdl2::video::{ GLProfile, Window };

use gl::{self, GlContext};
use builder::ContextBuilder;
use error::{AppResult, AppError};


fn find_sdl_gfx_driver() -> AppResult<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" || item.name == "vulkan" {
            return Ok(index as u32);
        }
    }
    Err(AppError::WindowError(
        String::from("Failed top find Graphics Driver"),
    ))
}

pub struct Context {
    gl_context: GlContext,
    event_pump: EventPump,
    window: Window,
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

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let window = video_subsystem
            .window("Window", 800, 600)
            .opengl()
            .build()
            .unwrap();

        let ctx = window.gl_create_context().unwrap();
        gl::raw::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

        debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
        debug_assert_eq!(gl_attr.context_version(), (3, 3));

        let mut event_pump = sdl_context.event_pump().unwrap();

        Ok(Context {
            gl_context: Default::default(),
            event_pump,
            window,
        })
    }

    pub fn poll_iter(&mut self) -> sdl2::event::EventPollIterator {
        self.event_pump.poll_iter()
    }

    pub fn blip(&mut self) {
        self.gl_context.clear();
        self.window.gl_swap_window();
    }
}
