pub mod raw;
pub mod program;
pub mod error;

use sdl2;

pub struct GlContext {
    pub sdl_gl: sdl2::video::GLContext,
}

impl GlContext {
    pub fn new(sdl_gl: sdl2::video::GLContext) -> GlContext {
        GlContext { sdl_gl }
    }
}

/// All OpenGL objects have an id which uses to
/// tell the driver to perform commands on them.
/// However, sometimes questions needed to be asked
/// about an object after original abstraction has
/// been Dropped
pub trait GlObject {
    /// Gets the Id of the GlObejct
    fn as_gl_id(&self) -> raw::types::GLuint;
}
