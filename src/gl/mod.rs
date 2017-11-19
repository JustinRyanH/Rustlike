use std::{ops, mem};
use std::os::raw::c_void;

pub mod raw;
pub mod program;
pub mod error;
pub mod buffer;
pub mod vertex;
pub mod example;

use sdl2;

use self::raw::types::*;
use self::buffer::BoundGlBuffer;
use self::vertex::VertexAttributes;
use error::AppResult;

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
    /// use rustlike::gl::AttributeKind;
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

impl From<f32> for AttributeKind {
    fn from(_: f32) -> AttributeKind {
        AttributeKind::Float
    }
}

impl From<i32> for AttributeKind {
    fn from(_: i32) -> AttributeKind {
        AttributeKind::Int
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

pub struct AttributeValue<T>
where
    T: Into<AttributeKind>,
{
    pub value: T,
    kind: AttributeKind,
}

impl From<i32> for AttributeValue<i32> {
    fn from(value: i32) -> AttributeValue<i32> {
        AttributeValue {
            value,
            kind: value.into(),
        }
    }
}

impl From<f32> for AttributeValue<f32> {
    fn from(value: f32) -> AttributeValue<f32> {
        AttributeValue {
            value,
            kind: value.into(),
        }
    }
}

impl<T> Into<AttributeKind> for AttributeValue<T>
where
    T: Into<AttributeKind>,
{
    fn into(self) -> AttributeKind {
        self.kind
    }
}

/// # Example
/// ```
///  use rustlike::gl::AttributeValue;
///  let attr_value: AttributeValue<i32> = 1.into();
///  assert_eq!(1, *attr_value);
/// ```
impl<T> ops::Deref for AttributeValue<T>
where
    T: Into<AttributeKind>,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

pub trait IntoAttributeCollection<A>: Into<AttributeCollection<A>>
where
    A: VertexAttributes
{
}

pub trait BindableCollection<A>
where
    A: VertexAttributes,
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

#[derive(Debug)]
pub struct AttributeCollection<A>
where
    A: VertexAttributes,
{
    collection: Vec<A>,
}

impl<A> BindableCollection<A> for AttributeCollection<A>
where
    A: VertexAttributes,
{
    #[inline]
    unsafe fn bind_to_buffer(&self, bounded_buffer: &BoundGlBuffer) -> AppResult<()> {
        let size = (self.collection.len() * mem::size_of::<A>()) as isize;
        raw::BufferData(
            bounded_buffer.kind().into(),
            size,
            &self.collection[0] as *const A as *const c_void,
            // TODO: This needs to be configurable. Likely to the buffer
            raw::STATIC_DRAW,
        );
        Ok(())
    }

    #[inline]
    unsafe fn describe_to_buffer(&self, bounded_buffer: &BoundGlBuffer) {
        for (index, attribute) in A::attributes().iter().enumerate() {
            attribute.describe_to_gl(bounded_buffer, index as u32)
        }
    }
}
