extern crate sdl2;

pub mod raw;
pub mod errors;
pub mod attributes;
pub mod program;
pub mod buffer;
// pub mod example;

use buffer::BoundGlBuffer;
use errors::GlResult;

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
    unsafe fn kind(&self) -> GlResult<AttributeKind> {
        Ok(
            A::attributes()
                .first()
                .ok_or(errors::GlError::AttributeError(
                    format!("Vertex must have at least one attribute"),
                ))?
                .kind(),
        )
    }
    unsafe fn bind_to_buffer(&self, bounded_buffer: &BoundGlBuffer) -> GlResult<()>;
    unsafe fn describe_to_buffer(&self, bounded_buffer: &BoundGlBuffer);
}
