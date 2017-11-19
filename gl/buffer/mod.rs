mod array_object;
mod buffer_object;

pub use self::array_object::*;
pub use self::buffer_object::*;

use error::AppResult;
use gl::{self, BindableCollection, GlObject};
use gl::attributes::AttributeCollection;

pub struct BufferConfiguration<A>
where
    A: gl::attributes::DescribeAttributes,
{
    vertices: AttributeCollection<A>
}

impl<A> BufferConfiguration<A>
    where
    A: gl::attributes::DescribeAttributes,
{
    pub fn new(vertices: AttributeCollection<A>) -> BufferConfiguration<A> {
        BufferConfiguration {
            vertices,
        }
    }

    pub fn build(self) -> AppResult<BufferObject> {
        let mut vbo = GlBuffer::new(BufferKind::Array);
        let mut vao = VertexArrayObject::new();

        unsafe {
            let bounded_vao = vao.bind();
            // TODO: There needs to be a global GlContext that prevents this guy
            // from binding if someone of the same type is bounded. Otherwise build
            // errors.
            let bounded_vbo = vbo.bind(Some(&bounded_vao));
            self.vertices.bind_to_buffer(&bounded_vbo)?;
            self.vertices.describe_to_buffer(&bounded_vbo);
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