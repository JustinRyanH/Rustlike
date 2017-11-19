use std::{ptr, mem, ops};
use std::os::raw::c_void;


use BindableCollection;
use raw;
use raw::types::*;
use errors::GlResult;
use buffer::BoundGlBuffer;

pub trait DescribeAttributes: Clone {
    unsafe fn attributes() -> Vec<Attribute>
    where
        Self: Sized;
}

#[derive(Clone, Copy, Debug)]
pub enum AttributeSize {
    One,
    Two,
    Three,
    Four,
}

impl Into<GLint> for AttributeSize {
    fn into(self) -> GLint {
        match self {
            AttributeSize::One => 1,
            AttributeSize::Two => 2,
            AttributeSize::Three => 3,
            AttributeSize::Four => 4,
        }
    }
}


#[derive(Clone, Debug)]
pub struct Attribute {
    size: AttributeSize,
    kind: AttributeKind,
    normalized: bool,
    stride: usize,
}

impl Attribute {
    pub fn new(
        size: AttributeSize,
        kind: AttributeKind,
        normalized: bool,
        stride: usize,
    ) -> Attribute {
        Attribute {
            size,
            kind,
            normalized,
            stride,
        }
    }

    pub fn kind(&self) -> AttributeKind {
        self.kind
    }

    pub fn normalized(&self) -> GLboolean {
        match self.normalized {
            true => raw::TRUE,
            false => raw::FALSE,
        }
    }

    pub unsafe fn describe_to_gl<'a>(&self, _: &BoundGlBuffer<'a>, index: u32) {
        let size: GLint = self.size.into();
        raw::VertexAttribPointer(
            index,
            size,
            self.kind.into(),
            self.normalized(),
            self.stride as GLsizei,
            ptr::null(),
        );
        raw::EnableVertexAttribArray(index);
    }
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
            AttributeKind::Byte => raw::BYTE,
            AttributeKind::UnsignedByte => raw::UNSIGNED_BYTE,
            AttributeKind::Float => raw::FLOAT,
            AttributeKind::Double => raw::DOUBLE,
            AttributeKind::Short => raw::SHORT,
            AttributeKind::UnsignedShort => raw::UNSIGNED_SHORT,
            AttributeKind::Int => raw::INT,
            AttributeKind::UnsignedInt => raw::UNSIGNED_INT,
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
    A: DescribeAttributes
{
}

#[derive(Debug)]
pub struct AttributeCollection<A>
where
    A: DescribeAttributes,
{
    collection: Vec<A>,
}

impl<A> AttributeCollection<A>
where
    A: DescribeAttributes,
{
    pub fn new(collection: Vec<A>) -> AttributeCollection<A> {
        AttributeCollection { collection }
    }
}

impl<A> BindableCollection<A> for AttributeCollection<A>
where
    A: DescribeAttributes,
{
    #[inline]
    unsafe fn bind_to_buffer(&self, bounded_buffer: &BoundGlBuffer) -> GlResult<()> {
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
