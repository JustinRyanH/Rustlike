//! Handles memory allocation for the GPU as well as transferring the Description of
//! of the data, so OpenGL can communicate to it.
use std::ptr;

mod array_object;
mod buffer_object;

pub use self::array_object::*;
pub use self::buffer_object::*;

use GlObject;
use raw;
use errors::{GlResult, GlError};
use attributes::{self, AttributeKind};


/// Buffers Reference can be build from objects that implement this
pub trait BufferBuilder {
    /// Builder a BufferReference
    fn build(self) -> GlResult<BufferReference>;
}

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

    /// Creates a Configuration that takes indices
    pub fn with_index(self, indices: Vec<u32>) -> BufferConfigurationWithIndex<A> {
        BufferConfigurationWithIndex {
            vertices: self.vertices,
            indices,
        }
    }
}

impl<A> BufferBuilder for BufferConfiguration<A>
where
    A: attributes::DescribeAttributes,
{
    /// Builds the Buffer Object
    fn build(self) -> GlResult<BufferReference> {
        // TODO: There needs to be a global GlContext that prevents this guy
        // from binding if someone of the same type is bounded. Otherwise build
        // errors.
        let mut vbo = BufferObject::new(BufferKind::Array);
        let mut vao = VertexArrayObject::new();

        unsafe {
            let mut buffers = vec![&mut vbo];
            let bounded_vao = vao.bind(buffers.as_mut_slice())?;
            match bounded_vao.vertex_buffer_object() {
                Some(ref bounded_vbo) => bounded_vbo.bind_structure_to_buffer(&self.vertices)?,
                None => {
                    return Err(GlError::BindingError(
                        "No VBO Found. Vertex Buffer Object is required to build a BufferObject"
                            .into(),
                    ))
                }
            }
            bounded_vao.describe_attributes(A::attributes())?;
        };

        Ok(BufferReference {
            vao,
            vbo,
            ebo: None,
            indices: self.vertices.len(),
        })
    }
}

/// Allows for Configuration and Building Buffer Objects with Indices
///
/// Generally the data it holds will get consumed
/// and deallocated after it has successfully been
/// consumed by the OpenGL driver.
pub struct BufferConfigurationWithIndex<A>
where
    A: attributes::DescribeAttributes,
{
    vertices: Vec<A>,
    indices: Vec<u32>,
}

impl<A> BufferBuilder for BufferConfigurationWithIndex<A>
where
    A: attributes::DescribeAttributes,
{
    /// Builds the Buffer Object
    fn build(self) -> GlResult<BufferReference> {
        // TODO: There needs to be a global GlContext that prevents this guy
        // from binding if someone of the same type is bounded. Otherwise build
        // errors.
        let mut vbo = BufferObject::new(BufferKind::Array);
        let mut ebo = BufferObject::new(BufferKind::ElementArrayBuffer);
        let mut vao = VertexArrayObject::new();

        unsafe {
            let mut buffers = vec![&mut vbo, &mut ebo];
            let bounded_vao = vao.bind(buffers.as_mut_slice())?;
            match bounded_vao.vertex_buffer_object() {
                Some(ref bounded_vbo) => bounded_vbo.bind_structure_to_buffer(&self.vertices)?,
                None => {
                    return Err(GlError::BindingError(
                        "No VBO Found. Vertex Buffer Object is required to build a BufferObject"
                            .into(),
                    ))
                }
            }

            match bounded_vao.element_array_buffer() {
                Some(ref bounded_ebo) => bounded_ebo.bind_flat_array_to_buffer(&self.indices)?,
                None => {
                    return Err(GlError::BindingError(
                        "No EBO Found. Vertex Buffer Object is required to build a BufferObject"
                            .into(),
                    ))
                }
            }
            bounded_vao.describe_attributes(A::attributes())?;
        };

        Ok(BufferReference {
            vao,
            vbo,
            ebo: Some((ebo, AttributeKind::UnsignedInt)),
            indices: self.indices.len(),
        })
    }
}


/// Reference to OpenGL buffer.
#[derive(Debug)]
pub struct BufferReference {
    vao: VertexArrayObject,
    vbo: BufferObject,
    ebo: Option<(BufferObject, AttributeKind)>,
    indices: usize,
}


impl BufferReference {
    /// Draws the BufferReference to the Screen
    /// TODO: Yeah this needs a bit of work
    pub fn draw(&self) -> GlResult<()> {
        unsafe {
            raw::BindVertexArray(self.vao.as_gl_id());
            match self.ebo {
                Some((_, kind)) => {
                    raw::DrawElements(
                        raw::TRIANGLES,
                        self.indices as i32,
                        kind.into(),
                        ptr::null(),
                    );
                }
                None => {
                    raw::DrawArrays(raw::TRIANGLES, 0, self.indices as i32);
                }
            }
        }
        Ok(())
    }
}
