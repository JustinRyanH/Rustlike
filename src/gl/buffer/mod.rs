mod array_object;
mod buffer_object;

use std::{mem};

use std::os::raw::c_void;

pub use self::array_object::*;
pub use self::buffer_object::*;

use error::AppResult;
use gl;
use gl::vertex::VertexCollection;
use gl::GlObject;
use gl::raw::types::*;

pub struct BufferConfiguration<T>
where
    T: gl::vertex::VertexAttributes,
{
    vertices: gl::vertex::VertexCollection<T>,
}

impl<T> BufferConfiguration<T>
where
    T: gl::vertex::VertexAttributes,
{
    pub fn new<K>(vertices: K) -> BufferConfiguration<T>
    where
        K: Into<VertexCollection<T>>,
    {
        BufferConfiguration { vertices: vertices.into() }
    }

    // TODO: Use AppResult, this should valid the health.
    pub fn build(self) -> AppResult<BufferObject> {
        let mut vbo = GlBuffer::new(BufferKind::Array);
        let mut vao = VertexArrayObject::new();

        unsafe {
            let vec_vert: Vec<f32> = self.vertices.into();
            let slice = vec_vert.as_slice();

            let bounded_vao = vao.bind();
            let bounded_vbo = vbo.bind(Some(&bounded_vao));
            gl::raw::BufferData(
                bounded_vbo.kind().into(),
                (slice.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &slice[0] as *const f32 as *const c_void,
                gl::raw::STATIC_DRAW,
            );

            bounded_vbo.describe_attributes(T::attributes())
        }

        Ok(BufferObject {
            vao,
            vbo,
            ebo: None,
        })
    }
}

#[derive(Debug)]
pub struct BufferObject {
    vao: VertexArrayObject,
    vbo: GlBuffer,
    ebo: Option<GlBuffer>,
}


impl BufferObject {
    pub fn draw(&self) -> AppResult<()> {
        unsafe {
            gl::raw::BindVertexArray(self.vao.as_gl_id());
            gl::raw::DrawArrays(gl::raw::TRIANGLES, 0, 3)
        }
        Ok(())
    }
}
