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

/// Abstract Representation of OpenGL Vertex
/// Array that tell the GPU to clean itself up
/// when it is goes out of use
pub struct VertexArray(GLuint, i32);

impl VertexArray {
    pub fn new() -> VertexArray {
        let mut vao = 0;
        unsafe {
            gl::raw::GenVertexArrays(1, &mut vao);
        }
        VertexArray(vao, 1)
    }

    pub fn bind<F>(&mut self, f: F) -> AppResult<VertexBuffer>
    where
        F: Fn() -> AppResult<VertexBuffer>,
    {
        unsafe {
            gl::raw::BindVertexArray(self.0);
            let buffer = f();
            gl::raw::BindVertexArray(0);
            buffer
        }
    }
}

impl GlObject for VertexArray {
    fn as_gl_id(&self) -> GLuint {
        return self.0;
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { gl::raw::DeleteVertexArrays(1, &self.0) }
    }
}

pub struct BufferObject {
    vao: VertexArray,
    vbo: VertexBuffer,
}

impl BufferObject {
    pub fn new(builder: &mut VertexBufferBuilder) -> AppResult<BufferObject> {
        let mut vao = VertexArray::new();
        let vbo = vao.bind(move || Ok(builder.build()?))?;

        Ok(BufferObject { vao, vbo })
    }

    pub fn vbo(&self) -> &VertexBuffer {
        &self.vbo
    }

    pub fn vao(&self) -> &VertexArray {
        &self.vao
    }

    pub fn draw(&self) -> AppResult<()> {
        unsafe {
            gl::raw::BindVertexArray(self.vao.as_gl_id());
            gl::raw::DrawArrays(gl::raw::TRIANGLES, 0, 3);
        }
        Ok(())
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

    pub fn build(&self) -> AppResult<VertexBuffer> {
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
