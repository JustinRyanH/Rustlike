use super::GlObject;
use raw;
use raw::types::*;

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
    pub fn bind<'a>(&'a mut self) -> BoundVertexArrayObject<'a> {
        unsafe {
            raw::BindVertexArray(self.0);
        }
        BoundVertexArrayObject(self)
    }
}

impl GlObject for VertexArrayObject {
    #[inline]
    fn as_gl_id(&self) -> GLuint {
        return self.0
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
pub struct BoundVertexArrayObject<'a>(&'a VertexArrayObject);
impl<'a> Drop for BoundVertexArrayObject<'a> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            raw::BindVertexArray(0);
        }
    }
}
