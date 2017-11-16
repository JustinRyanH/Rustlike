mod array_object;
mod buffer_object;

use std::mem;

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
    indices: Option<Vec<usize>>,
}

impl<T> BufferConfiguration<T>
where
    T: gl::vertex::VertexAttributes,
{
    pub fn new<K>(vertices: K) -> BufferConfiguration<T>
    where
        K: Into<VertexCollection<T>>,
    {
        BufferConfiguration {
            vertices: vertices.into(),
            indices: None,
        }
    }

    pub fn with_index<L: Into<Vec<usize>>>(mut self, indices: L) -> BufferConfiguration<T> {
        self.indices = Some(indices.into());
        self
    }


    // TODO: Use this should validate the health.
    pub fn build(self) -> AppResult<BufferObject> {
        let mut vbo = GlBuffer::new(BufferKind::Array);
        let mut vao = VertexArrayObject::new();
        let mut ebo = match self.indices {
            Some(_) => Some(GlBuffer::new(BufferKind::ElementArrayBuffer)),
            None => None,
        };

        unsafe {
            let vec_vert: Vec<f32> = self.vertices.into();
            let slice = vec_vert.as_slice();

            let bounded_vao = vao.bind();
            let bounded_vbo = vbo.bind(Some(&bounded_vao));
            let bounded_ebo = match ebo {
                Some(ref mut e) => Some(e.bind(Some(&bounded_vao))),
                None => None,
            };

            bounded_vbo.load_data(slice);
            // match bounded_ebo {
            //     Some(ref mut b_ebo) => e_bo.load_data(self.indices.unwrap()),
            //     None => None,
            // }
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
