//! Handles memory allocation for the GPU as well as transferring the Description of
//! of the data, so OpenGL can communicate to it.
mod array_object;
mod buffer_object;

pub use self::array_object::*;
pub use self::buffer_object::*;

use GlObject;
use raw;
use errors::GlResult;
use attributes;

/// Allows for Configuration and Building Buffer Objects.
///
/// Generally the data it holds will get consumed
/// and deallocated after it has successfully been
/// consumed by the OpenGL driver.
pub struct BufferConfiguration<A>
where
    A: attributes::DescribeAttributes,
{
    vertices: Vec<A>,
}

impl<A> BufferConfiguration<A>
where
    A: attributes::DescribeAttributes,
{
    /// Creates a Configuration from an Array of describable structs
    pub fn new(vertices: Vec<A>) -> BufferConfiguration<A> {
        BufferConfiguration { vertices: vertices.into() }
    }

    /// Builds the Buffer Object
    pub fn build(self) -> GlResult<BufferReference> {
        let mut vbo = BufferObject::new(BufferKind::Array);
        let mut vao = VertexArrayObject::new();

        unsafe {
            let bounded_vao = vao.bind();
            // TODO: There needs to be a global GlContext that prevents this guy
            // from binding if someone of the same type is bounded. Otherwise build
            // errors.
            let bounded_vbo = vbo.bind(Some(&bounded_vao));
            bounded_vbo.bind_to_buffer(&self.vertices)?;
            bounded_vbo.describe_to_buffer(&self.vertices)?;
        }

        Ok(BufferReference {
            vao,
            vbo,
            ebo: None,
        })
    }
}


/// Reference to OpenGL buffer.
#[derive(Debug)]
pub struct BufferReference {
    vao: VertexArrayObject,
    vbo: BufferObject,
    ebo: Option<BufferObject>,
}


impl BufferReference {
    /// Draws the BufferReference to the Screen
    /// TODO: Yeah this needs a bit of work
    pub fn draw(&self) -> GlResult<()> {
        unsafe {
            raw::BindVertexArray(self.vao.as_gl_id());
            raw::DrawArrays(raw::TRIANGLES, 0, 3)
        }
        Ok(())
    }
}

