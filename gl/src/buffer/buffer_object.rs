use std::mem;
use std::os::raw::c_void;

use GlObject;
use raw;
use raw::types::*;
use errors::GlResult;
use attributes::{DescribeAttributes, AttributeKind};

/// Used to specify the kind of buffer a BufferObject is bound too.
///
/// [more](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBuffer.xhtml) from OpenGL API
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BufferKind {
    /// Vertex Array Object
    Array,
    /// Element Array Object
    ElementArrayBuffer,
}

impl Into<GLenum> for BufferKind {
    fn into(self) -> GLenum {
        match self {
            BufferKind::ElementArrayBuffer => raw::ELEMENT_ARRAY_BUFFER,
            BufferKind::Array => raw::ARRAY_BUFFER,
        }
    }
}

impl ToString for BufferKind {
    fn to_string(&self) -> String {
        match self {
            &BufferKind::ElementArrayBuffer => "ebo".into(),
            &BufferKind::Array => "vao".into(),
        }
    }
}

/// Reference to stored unformatted data on GPU.
///
/// [more](https://www.khronos.org/opengl/wiki/Buffer_Object) from OpenGL API
#[derive(Debug)]
pub struct BufferObject {
    glid: GLuint,
    len: i32,
    kind: BufferKind,
}

impl BufferObject {
    /// Generates new Buffer Object
    ///
    /// [more](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenBuffers.xhtml) from the OpenGL API
    pub fn new(kind: BufferKind) -> BufferObject {
        let mut glid = 0;
        unsafe {
            raw::GenBuffers(1, &mut glid);
        }
        BufferObject { glid, len: 1, kind }
    }

    /// Temporary Binds the BufferObject to the OpenGL context.
    /// Although it isn't required, if you have a Vertex Array Buffer
    /// bounded you should pass it through the BufferObject bounded, so
    /// that the binding and unbinding behavior becomes deterministic
    pub fn bind<'a>(&'a mut self) -> BoundBufferObject<'a> {
        unsafe {
            raw::BindBuffer(self.kind.into(), self.as_gl_id());
        }
        BoundBufferObject(self)
    }

    /// gets kind of Buffer Object
    pub fn kind(&self) -> BufferKind {
        self.kind
    }
}

impl GlObject for BufferObject {
    fn as_gl_id(&self) -> GLuint {
        return self.glid;
    }
}

impl Drop for BufferObject {
    fn drop(&mut self) {
        unsafe { raw::DeleteBuffers(self.len, &self.glid) }
    }
}


/// Is used to scope a OpenGL BindBuffer. It unbinds the buffer
/// when it goes out of scope.
///
/// created through [BufferObject](struct.BufferObject.html#method.bind)
#[derive(Debug)]
pub struct BoundBufferObject<'a>(&'a BufferObject);

impl<'a> BoundBufferObject<'a> {
    /// Rebinds the BufferObject in case someone has stepped on our toes
    unsafe fn rebind(&self) {
        raw::BindBuffer(self.0.kind.into(), self.0.as_gl_id());
    }
    /// gets kind of Buffer Object
    #[inline]
    pub fn kind(&self) -> BufferKind {
        self.0.kind
    }

    /// Loads describable structure into the Buffer.
    ///
    /// [more](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferData.xhtml) from OpenGL API
    #[inline]
    pub unsafe fn bind_structure_to_buffer<A>(&self, vertices: &Vec<A>) -> GlResult<()>
    where
        A: DescribeAttributes,
    {
        self.rebind();
        // TODO: This should be safe. We should do calls to verify data is good
        // then we can move the unsafe block inside the function
        let size = (vertices.len() * mem::size_of::<A>()) as isize;
        raw::BufferData(
            self.kind().into(),
            size,
            &vertices[0] as *const A as *const c_void,
            raw::STATIC_DRAW,
        );
        Ok(())
    }

    /// Loads passed valid into the Buffer.
    ///
    /// [more](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferData.xhtml) from OpenGL API
    #[inline]
    pub unsafe fn bind_flat_array_to_buffer<A>(&self, vec: &Vec<A>) -> GlResult<()>
    where
        A: Into<AttributeKind> + Clone,
    {
        self.rebind();
        let kind: AttributeKind = vec[0].clone().into();
        raw::BufferData(
            raw::ELEMENT_ARRAY_BUFFER,
            (vec.len() * kind.size_of()) as GLsizeiptr,
            &vec[0] as *const A as *const c_void,
            raw::STATIC_DRAW,
        );
        Ok(())
    }
}

impl<'a> Drop for BoundBufferObject<'a> {
    fn drop(&mut self) {
        unsafe { raw::BindBuffer(self.0.kind.into(), 0) }
    }
}
