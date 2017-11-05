use sdl2;

use gl;
use error::AppResult;

pub struct ContextBuilder {}

impl ContextBuilder {
    // Builds a new Context for the Application
    pub fn build(self) -> AppResult<Context> {
        Context::new()
    }
}

impl Default for ContextBuilder {
    fn default() -> ContextBuilder {
        ContextBuilder {}
    }
}

pub struct Context {
    sdl: sdl2::Sdl,
    window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
    gl_context: gl::GlContext,
}

impl Context {
    // Initializes Window, Events, and Graphics Contexts
    pub fn new() -> AppResult<Context> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;

        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        debug_assert_eq!(gl_attr.context_version(), (3, 3));
        // TODO: In house builder should just return this guy
        let window = video.window("Window", 800, 600).opengl().build()?;

        let gl_context = gl::GlContext::new(window.gl_create_context()?);
        gl::raw::load_with(|name| video.gl_get_proc_address(name) as *const _);

        let event_pump = sdl.event_pump()?;


        debug_assert_eq!(gl_attr.context_version(), (3, 3));
        Ok(Context {
            sdl,
            window,
            event_pump,
            gl_context,
        })
    }

    pub fn window(&self) -> &sdl2::video::Window {
        &self.window
    }

    pub fn present(&mut self) {
        self.window.gl_swap_window();
    }

    pub fn poll_iter(&mut self) -> sdl2::event::EventPollIterator {
        self.event_pump.poll_iter()
    }

    pub fn gl(&mut self) -> &gl::GlContext {
        &self.gl_context
    }

    pub fn sdl(&mut self) -> &sdl2::Sdl {
        &self.sdl
    }
}
