use gl::{self, GlObject};
use gl::raw::types::*;

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

    // TODO: This should take an Option for Array binding to for
    // user to specify kind binding
    pub fn bind<'a>(&'a mut self) -> BoundGlBuffer<'a> {
        unsafe {
            gl::raw::BindBuffer(self.kind.into(), self.as_gl_id());
        }
        BoundGlBuffer(self)
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

pub struct BoundGlBuffer<'a>(&'a GlBuffer);
impl<'a> BoundGlBuffer<'a> {
    pub fn kind(&self) -> BufferKind {
        self.0.kind
    }
}


impl<'a> Drop for BoundGlBuffer<'a> {
    fn drop(&mut self) {
        unsafe { gl::raw::BindBuffer(self.0.kind.into(), 0) }
    }
}
