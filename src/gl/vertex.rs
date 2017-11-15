use std::{mem, ptr};

use gl;
use gl::error::GlError;
use gl::raw::types::*;
use error::AppResult;

#[derive(Clone, Debug)]
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
}


pub trait VertexAttributes: Into<Vec<f32>> + Clone {
    fn attributes() -> Vec<Attribute>
    where
        Self: Sized;
}



#[derive(Clone)]
pub struct ExampleVertex {
    pub pos: [f32; 3],
}

impl VertexAttributes for ExampleVertex {
    #[inline]
    fn attributes() -> Vec<Attribute> {
        [
            Attribute::new(
                AttributeSize::Three,
                AttributeKind::Float,
                false,
                3 * mem::size_of::<GLfloat>() as usize,
            ),
        ].to_vec()
    }
}

// This is the un optimal method of turning
// the vertices into flat slice, whenever
// https://doc.rust-lang.org/std/slice/trait.SliceConcatExt.html
// becomes stable we will implement it as an alternative

impl Into<Vec<f32>> for ExampleVertex {
    fn into(self) -> Vec<f32> {
        [self.pos[0], self.pos[1], self.pos[2]].to_vec()
    }
}

pub struct VertexCollection<T>(Vec<T>)
where
    T: VertexAttributes;

impl<T> From<Vec<T>> for VertexCollection<T>
where
    T: VertexAttributes,
{
    fn from(v: Vec<T>) -> VertexCollection<T> {
        VertexCollection(v)
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
