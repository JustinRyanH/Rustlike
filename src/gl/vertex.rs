use std::ptr;
use std::fmt;

use gl::{self, AttributeKind};
use gl::buffer::BoundGlBuffer;
use gl::raw::types::*;

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
            true => gl::raw::TRUE,
            false => gl::raw::FALSE,
        }
    }

    pub unsafe fn describe_to_gl<'a>(&self, _: &BoundGlBuffer<'a>, index: u32) {
        let size: GLint = self.size.into();
        gl::raw::VertexAttribPointer(
            index,
            size,
            self.kind.into(),
            self.normalized(),
            (size * self.kind.size_of() as i32) as GLsizei,
            ptr::null(),
        );
        gl::raw::EnableVertexAttribArray(index);
    }
}

pub trait VertexAttributes: Into<Vec<f32>> + Clone + fmt::Debug {
    fn attributes() -> Vec<Attribute>
    where
        Self: Sized;

}
