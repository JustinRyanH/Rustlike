use std::{mem, ptr};

use gl;
use gl::error::GlError;
use gl::raw::types::*;
use error::AppResult;

pub trait VertexAttribute {
    fn define_attribute_pointer(index: usize) -> AppResult<()>;
    fn flatten(&self) -> Vec<f32>;
}

pub struct ExampleVertex {
    pos: [f32; 3],
}

impl VertexAttribute for ExampleVertex {
    fn define_attribute_pointer(index: usize) -> AppResult<()> {
        if index > 0 {
            return Err(
                GlError::AttributeError(
                    format!("{} is out of bounds for Vertex definitions", index),
                ).into(),
            );
        }
        unsafe {
            gl::raw::VertexAttribPointer(
                0,
                3,
                gl::raw::FLOAT,
                gl::raw::FALSE,
                3 * mem::size_of::<GLfloat>() as GLsizei,
                ptr::null(),
            );
            gl::raw::EnableVertexAttribArray(0);
        }
        Ok(())
    }

    fn flatten(&self) -> Vec<f32> {
        self.pos.to_vec()
    }
}


pub struct VertexCollection(Vec<ExampleVertex>);

impl Into<Vec<f32>> for VertexCollection {
    fn into(self) -> Vec<f32> {
        self.0.iter().flat_map(|e| e.flatten()).collect()
    }
}
