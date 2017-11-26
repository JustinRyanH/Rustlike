use std::collections::HashMap;
use std::mem;
use std::os::raw::c_void;

use raw;
use raw::types::*;

use GlObject;
use errors::{GlError, GlResult};
use buffer::{BoundBufferObject, BufferObject, BufferKind};
use attributes::Attribute;

/// Reference to Vertex description of data stored inside of Buffer Object
///
/// [more](https://www.khronos.org/opengl/wiki/Vertex_Specification#Vertex_Array_Object) from the OpenGL Wiki
#[derive(Debug)]
pub struct VertexArrayObject(GLuint, i32);

impl VertexArrayObject {
    /// Generates a new Vertex Array Object
    ///
    /// [more](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenVertexArrays.xhtml) from OpenGL API
    pub fn new() -> VertexArrayObject {
        let mut vao = 0;
        unsafe {
            raw::GenVertexArrays(1, &mut vao);
        }
        VertexArrayObject(vao, 1)
    }

    /// Returns a Bounded VertexArrayObject. It will be unbounded when
    /// the Object goes out of scope.
    ///
    /// [more](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexArray.xhtml) from the OpenGL API
    #[inline]
    pub fn bind<'a>(
        &'a mut self,
        buffers: &'a mut [&'a mut BufferObject],
    ) -> GlResult<BoundVertexArrayObject<'a>> {
        let mut bounded_buffers: HashMap<BufferKind, BoundBufferObject> = HashMap::new();

        unsafe {
            raw::BindVertexArray(self.0);
            for buffer in &mut buffers.iter_mut() {
                let kind = buffer.kind();
                let bounded = buffer.bind();
                match bounded_buffers.insert(kind, bounded) {
                    Some(_) => {
                        return Err(GlError::BindingError(format!(
                            "Only one {} can be bounded to a vertex array object at a time",
                            kind.to_string()
                        )))
                    }
                    None => (),
                };
            }
        };
        Ok(BoundVertexArrayObject {
            vao: self,
            bounded_buffers,
        })
    }
}

impl GlObject for VertexArrayObject {
    #[inline]
    fn as_gl_id(&self) -> GLuint {
        return self.0;
    }
}

impl Drop for VertexArrayObject {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            raw::DeleteVertexArrays(self.1, &mut self.0);
        }
    }
}

/// Is used to scope a OpenGL BindVertexArrays. It unbinds the buffer
/// when it goes out of scope.
///
/// created through [VertexArrayObject](struct.VertexArrayObject.html#method.bind)
#[derive(Debug)]
pub struct BoundVertexArrayObject<'a> {
    vao: &'a VertexArrayObject,
    bounded_buffers: HashMap<BufferKind, BoundBufferObject<'a>>,
}

impl<'a> BoundVertexArrayObject<'a> {

    /// Gets the Element Array Buffer bounded
    pub fn element_array_buffer(&self) -> Option<&BoundBufferObject<'a>> {
        self.bounded_buffers.get(&BufferKind::ElementArrayBuffer)
    }

    /// Gets the Vertex Array Object if bounded
    pub fn vertex_buffer_object(&self) -> Option<&BoundBufferObject<'a>> {
        self.bounded_buffers.get(&BufferKind::Array)
    }

    /// Describe to OpenGL how to grok the passed data.
    ///
    /// [more](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribPointer.xhtml) from the OpenGL API
        #[inline]
    pub unsafe fn describe_attributes(&self, attributes: Vec<Attribute>) -> GlResult<()>
    {
        // TODO: This should be safe. We should do calls to verify data is good
        // then we can move the unsafe block inside the function
        for (index, attribute) in attributes.iter().enumerate() {
            let size: GLint = attribute.size().into();
            raw::VertexAttribPointer(
                index as u32,
                size,
                attribute.kind().into(),
                attribute.normalized(),
                attribute.stride() as GLsizei,
                attribute.point() as *const c_void,
            );
            raw::EnableVertexAttribArray(index as u32);
        }
        Ok(())
    }
}
impl<'a> Drop for BoundVertexArrayObject<'a> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            raw::BindVertexArray(0);
            /// The EBO needs to be dropped after the VertexArray is unbounded.
            /// otherwise the buffer will be unbounded from the VAO.
            for (_, bounded_buffer) in self.bounded_buffers.iter_mut() {
                mem::drop(bounded_buffer);
            }
        }
    }
}
