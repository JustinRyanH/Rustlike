use std::os::raw::c_void;

use gl::{self, GlObject};
use gl::raw::types::*;
use gl::AttributeKind;
use gl::vertex::{Attribute};
use gl::buffer::BoundVertexArrayObject;

#[derive(Clone, Copy, Debug)]
pub enum BufferKind {
    Array,
    ElementArrayBuffer,
}

impl Into<GLenum> for BufferKind {
    fn into(self) -> GLenum {
        match self {
            BufferKind::ElementArrayBuffer => gl::raw::ELEMENT_ARRAY_BUFFER,
            BufferKind::Array => gl::raw::ARRAY_BUFFER,
        }
    }
}

#[derive(Debug)]
pub struct GlBuffer {
    glid: GLuint,
    len: i32,
    kind: BufferKind,
}

impl GlBuffer {
    pub fn new(kind: BufferKind) -> GlBuffer {
        let mut glid = 0;
        unsafe {
            gl::raw::GenBuffers(1, &mut glid);
        }
        GlBuffer { glid, len: 1, kind }
    }

    /// Temporary Binds the GlBuffer to the OpenGL context.
    /// Although it isn't required, if you have a Vertex Array Buffer
    /// bounded you should pass it through the GlBuffer bounded, so
    /// that the binding and unbinding behavior is more deterministic
    pub fn bind<'a>(
        &'a mut self,
        vao: Option<&'a BoundVertexArrayObject<'a>>,
    ) -> BoundGlBuffer<'a> {
        unsafe {
            gl::raw::BindBuffer(self.kind.into(), self.as_gl_id());
        }
        BoundGlBuffer {
            vbo: self,
            _vao: vao,
        }
    }

    pub fn kind(&self) -> BufferKind {
        self.kind
    }
}

impl GlObject for GlBuffer {
    fn as_gl_id(&self) -> GLuint {
        return self.glid;
    }
}

impl Drop for GlBuffer {
    fn drop(&mut self) {
        unsafe { gl::raw::DeleteBuffers(self.len, &self.glid) }
    }
}

#[derive(Debug)]
pub struct BoundGlBuffer<'a> {
    vbo: &'a GlBuffer,
    _vao: Option<&'a BoundVertexArrayObject<'a>>,
}

impl<'a> BoundGlBuffer<'a> {
    #[inline]
    pub fn kind(&self) -> BufferKind {
        self.vbo.kind
    }

    #[inline]
    pub unsafe fn describe_attributes(&self, attrs: Vec<Attribute>) {
        for (index, attribute) in attrs.iter().enumerate() {
            attribute.describe_to_gl(self, index as u32)
        }
    }

    #[inline]
    pub unsafe fn load_data(&self, data: &[f32]) {
        // TODO: This can't just be floats. This needs to be describable
        gl::raw::BufferData(
            self.kind().into(),
            (data.len() * AttributeKind::Float.size_of()) as GLsizeiptr,
            &data[0] as *const f32 as *const c_void,
            gl::raw::STATIC_DRAW,
        );
    }
}


impl<'a> Drop for BoundGlBuffer<'a> {
    fn drop(&mut self) {
        unsafe { gl::raw::BindBuffer(self.vbo.kind.into(), 0) }
    }
}
