use gl::{self, GlObject};
use gl::raw::types::*;

/// Abstract Representation of OpenGL Vertex
/// Array that tell the GPU to clean itself up
/// when it is goes out of use
#[derive(Debug)]
pub struct VertexArrayObject(GLuint, i32);

impl VertexArrayObject {
    pub fn new() -> VertexArrayObject {
        let mut vao = 0;
        unsafe {
            gl::raw::GenVertexArrays(1, &mut vao);
        }
        VertexArrayObject(vao, 1)
    }

    /// Returns a Bounded VertexArrayObject. It will be unbounded when
    /// the Object goes out of scope.
    #[inline]
    pub fn bind<'a>(&'a mut self) -> BoundedVertexArrayObject<'a> {
        unsafe {
            gl::raw::BindVertexArray(self.0);
        }
        BoundedVertexArrayObject(self)
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
            gl::raw::DeleteVertexArrays(self.1, &mut self.0);
        }
    }
}

pub struct BoundedVertexArrayObject<'a>(&'a VertexArrayObject);
impl<'a> Drop for BoundedVertexArrayObject<'a> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            gl::raw::BindVertexArray(0);
        }
    }
}
