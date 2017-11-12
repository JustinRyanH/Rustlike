mod array_object;
mod buffer_object;

use std::{mem};

use std::os::raw::c_void;

pub use self::array_object::*;
pub use self::buffer_object::*;

use gl;
use gl::GlObject;
use gl::raw::types::*;

pub struct BufferConfiguration {
    vertices: Vec<f32>,
}

impl BufferConfiguration {
    pub fn new<T: Into<Vec<f32>>>(vertices: T) -> BufferConfiguration {
        BufferConfiguration { vertices: vertices.into() }
    }

    // TODO: Use AppResult, this should valid the health. 
    pub fn build(self) -> BufferObject {
        let mut vbo = GlBuffer::new(BufferKind::Array);
        let mut vao = VertexArrayObject::new();

        {
            let mut bounded_vao = vao.bind();
            let mut bounded_vbo = vbo.bind();

            unsafe {
                gl::raw::BufferData(
                    bounded_vbo.kind().into(),
                    (self.vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                    &self.vertices[0] as *const f32 as *const c_void,
                    gl::raw::STATIC_DRAW,
                );
            }


        }
        BufferObject::UnindexedVertexBuffer(vao, vbo)
    }
}


#[derive(Debug)]
pub enum BufferObject {
    VertexBuffer(VertexArrayObject, GlBuffer, GlBuffer),
    UnindexedVertexBuffer(VertexArrayObject, GlBuffer),
}

impl BufferObject {
    pub fn draw(&mut self) {
        let ref vao = match *self {
            BufferObject::VertexBuffer(ref vao, _, _) => vao,
            BufferObject::UnindexedVertexBuffer(ref vao, _) => vao,
        };
        unsafe {
            gl::raw::BindVertexArray(vao.as_gl_id());
            gl::raw::DrawArrays(gl::raw::TRIANGLES, 0, 3)
        }
    }
}
