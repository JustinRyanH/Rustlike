pub mod raw;
pub mod program;
pub mod error;
pub mod buffer;
pub mod attributes;
pub mod example;

use sdl2;

use self::buffer::BoundGlBuffer;
use error::AppResult;

pub use self::attributes::{AttributeKind, DescribeAttributes};

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


pub trait BindableCollection<A>
where
    A: DescribeAttributes,
{
    #[inline]
    unsafe fn kind(&self) -> AppResult<AttributeKind> {
        Ok(
            A::attributes()
                .first()
                .ok_or(format!("Vertex must have at least one attribute"))?
                .kind(),
        )
    }
    unsafe fn bind_to_buffer(&self, bounded_buffer: &BoundGlBuffer) -> AppResult<()>;
    unsafe fn describe_to_buffer(&self, bounded_buffer: &BoundGlBuffer);
}