use std::mem;

pub mod raw;
pub mod program;
pub mod error;
pub mod buffer;
pub mod vertex;

use sdl2;

use self::raw::types::*;

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

#[derive(Clone, Copy, Debug)]
pub enum AttributeKind {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Float,
    Double,
}


impl AttributeKind {
    /// Returns the size of attribute in bytes as used by OpenGL
    /// #Example
    /// ```
    /// use rustlike::gl::vertex::AttributeKind;
    ///
    /// assert_eq!(1, AttributeKind::Byte.size_of());
    ///
    /// ```
    pub fn size_of(self) -> usize {
        match self {
            AttributeKind::Byte => mem::size_of::<GLbyte>() as usize,
            AttributeKind::UnsignedByte => mem::size_of::<GLubyte>() as usize,
            AttributeKind::Short => mem::size_of::<GLshort>() as usize,
            AttributeKind::UnsignedShort => mem::size_of::<GLushort>() as usize,
            AttributeKind::Int => mem::size_of::<GLint>() as usize,
            AttributeKind::UnsignedInt => mem::size_of::<GLuint>() as usize,
            AttributeKind::Float => mem::size_of::<GLfloat>() as usize,
            AttributeKind::Double => mem::size_of::<GLdouble>() as usize,
        }
    }
}

impl Into<GLenum> for AttributeKind {
    fn into(self) -> GLenum {
        match self {
            AttributeKind::Byte => self::raw::BYTE,
            AttributeKind::UnsignedByte => self::raw::UNSIGNED_BYTE,
            AttributeKind::Float => self::raw::FLOAT,
            AttributeKind::Double => self::raw::DOUBLE,
            AttributeKind::Short => self::raw::SHORT,
            AttributeKind::UnsignedShort => self::raw::UNSIGNED_SHORT,
            AttributeKind::Int => self::raw::INT,
            AttributeKind::UnsignedInt => self::raw::UNSIGNED_INT,
        }
    }
}
