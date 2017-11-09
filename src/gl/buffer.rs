use std::{mem, ptr};
use std::os::raw::c_void;

use gl::{self, GlObject};
use gl::raw::types::*;
use error::AppResult;

#[derive(Debug, Clone, Copy)]
pub enum DrawKind {
    Static,
    Stream,
    Dynamic,
}

impl Into<GLenum> for DrawKind {
    fn into(self) -> GLenum {
        match self {
            DrawKind::Static => gl::raw::STATIC_DRAW,
            DrawKind::Dynamic => gl::raw::DYNAMIC_DRAW,
            DrawKind::Stream => gl::raw::STREAM_DRAW,
        }
    }
}

pub struct BufferObject {
    glid: GLuint,
    buffer: VertexBuffer,
}

impl BufferObject {
    pub fn new(builder: VertexBufferBuilder) -> AppResult<BufferObject> {
        let (glid, buffer) = unsafe {
            let mut vao = 0;
            /// TODO: Have this clean up the Vertex Array if failure Happens
            gl::raw::GenVertexArrays(1, &mut vao);
            gl::raw::BindVertexArray(vao);
            let buffer = builder.build()?;
            gl::raw::BindVertexArray(0);

            (vao, buffer)
        };

        Ok(BufferObject { glid, buffer })
    }

    pub fn bind(&self) {
        unsafe { gl::raw::BindVertexArray(self.glid) }
    }

    pub fn get_buffer(&self) -> &VertexBuffer {
        &self.buffer
    }
}

impl GlObject for BufferObject {
    fn as_gl_id(&self) -> GLuint {
        return self.glid;
    }
}

pub struct VertexBuffer(GLuint);

impl GlObject for VertexBuffer {
    fn as_gl_id(&self) -> GLuint {
        return self.0;
    }
}

/// The VertexBuffer holds the Construction data.
pub struct VertexBufferBuilder {
    data: Vec<f32>,
}

impl VertexBufferBuilder {
    pub fn new<T: Into<Vec<f32>>>(data: T) -> VertexBufferBuilder {
        return VertexBufferBuilder { data: data.into() };
    }

    pub fn build(self) -> AppResult<VertexBuffer> {
        let vbo = unsafe {
            let mut id = 0;
            gl::raw::GenBuffers(1, &mut id);
            gl::raw::BindBuffer(gl::raw::ARRAY_BUFFER, id);
            let data = self.data.as_slice();
            gl::raw::BufferData(
                gl::raw::ARRAY_BUFFER,
                (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &data[0] as *const f32 as *const c_void,
                gl::raw::STATIC_DRAW,
            );

            gl::raw::VertexAttribPointer(
                0,
                3,
                gl::raw::FLOAT,
                gl::raw::FALSE,
                3 * mem::size_of::<GLfloat>() as GLsizei,
                ptr::null(),
            );
            gl::raw::EnableVertexAttribArray(0);
            gl::raw::BindBuffer(gl::raw::ARRAY_BUFFER, 0);
            id
        };
        return Ok(VertexBuffer(vbo));
    }
}
