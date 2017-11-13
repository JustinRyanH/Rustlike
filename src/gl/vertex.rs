use std::{mem, ptr};

use gl;
use gl::error::GlError;
use gl::raw::types::*;
use error::AppResult;

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


pub enum AttributeKind {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    HalfFloat,
    Float,
    Double,
}

impl Into<GLenum> for AttributeKind {
    fn into(self) -> GLenum {
        match self {
            AttributeKind::Byte => gl::raw::BYTE,
            AttributeKind::UnsignedByte => gl::raw::UNSIGNED_BYTE,
            AttributeKind::Float => gl::raw::FLOAT,
            AttributeKind::Double => gl::raw::DOUBLE,
            AttributeKind::HalfFloat => gl::raw::HALF_FLOAT,
            AttributeKind::Short => gl::raw::SHORT,
            AttributeKind::UnsignedShort => gl::raw::UNSIGNED_SHORT,
            AttributeKind::Int => gl::raw::INT,
            AttributeKind::UnsignedInt => gl::raw::UNSIGNED_INT,
        }
    }
}

pub struct Attribute {
    size: AttributeSize,
    kind: AttributeKind,
    normalized: bool,
    stride: usize,
}


pub trait VertexAttributes: Into<Vec<f32>> + Clone {
    fn attributes() -> Vec<Attribute>
    where
        Self: Sized;
}



#[derive(Clone)]
pub struct ExampleVertex {
    pos: [f32; 3],
}

impl VertexAttributes for ExampleVertex {
    fn attributes() -> Vec<Attribute> {
        Vec::new()
    }
}

impl Into<Vec<f32>> for ExampleVertex {
    fn into(self) -> Vec<f32> {
        [self.pos[0], self.pos[1], self.pos[2]].to_vec()
    }
}

pub struct VertexCollection<T>(Vec<T>)
where
    T: VertexAttributes;
impl<T> VertexCollection<T>
where
    T: VertexAttributes,
{
    pub fn from_slice(value: &[T]) -> VertexCollection<T> {
        VertexCollection(value.to_vec())
    }
}

/// TODO: Test with Example
impl<T> Into<Vec<f32>> for VertexCollection<T>
where
    T: VertexAttributes,
{
    #[inline]
    fn into(self) -> Vec<f32> {
        let mut out: Vec<f32> = Vec::new();
        for element in self.0 {
            let f: Vec<f32> = element.into();
            out.extend(f);
        }
        out
    }
}
