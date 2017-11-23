use std::{mem, ops};
use std::os::raw::c_void;


use raw;
use raw::types::*;
use buffer::BoundGlBuffer;

pub trait DescribeAttributes: Clone {
    unsafe fn attributes() -> Vec<Attribute>
    where
        Self: Sized;
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
    point: usize,
}

impl Attribute {
    pub fn new(
        size: AttributeSize,
        kind: AttributeKind,
        normalized: bool,
        stride: usize,
        point: usize,
    ) -> Attribute {
        Attribute {
            size,
            kind,
            normalized,
            stride,
            point,
        }
    }

    pub fn kind(&self) -> AttributeKind {
        self.kind
    }

    pub fn size(&self) -> AttributeSize {
        self.size
    }

    pub fn normalized(&self) -> GLboolean {
        match self.normalized {
            true => raw::TRUE,
            false => raw::FALSE,
        }
    }

    pub fn stride(&self) -> usize {
        self.stride
    }

    pub fn point(&self) -> usize {
        self.point
    }

    pub unsafe fn describe_to_gl<'a>(&self, _: &BoundGlBuffer<'a>, index: u32) {
        let size: GLint = self.size.into();
        raw::VertexAttribPointer(
            index,
            size,
            self.kind.into(),
            self.normalized(),
            self.stride as GLsizei,
            self.point as *const c_void
        );
        raw::EnableVertexAttribArray(index);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
    /// use rl_gl::AttributeKind;
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

impl<T> ops::Deref for AttributeValue<T>
where
    T: Into<AttributeKind>,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}
