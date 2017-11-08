use std::{mem, ptr};

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

pub struct VertexArray {
    glid: GLuint,
    buffer: VertexBuffer,
}

impl VertexArray {
    pub fn new(builder: VertexBufferBuilder) -> AppResult<VertexArray> {
        let (glid, buffer) = unsafe {
            let mut vao = 0;
            /// TODO: Have this clean up the Vertex Array if failure Happens
            gl::raw::GenVertexArrays(1, &mut vao);
            gl::raw::BindVertexArray(vao);
            let buffer = builder.build()?;
            gl::raw::BindVertexArray(0);

            (vao, buffer)
        };

        Ok(VertexArray { glid, buffer })
    }

    pub fn bind(&self) {
        unsafe {
            gl::raw::BindVertexArray(self.glid)
        }
    }
}

impl GlObject for VertexArray {
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
}

impl VertexBufferBuilder {
    pub fn build(self) -> AppResult<VertexBuffer> {
        let vbo = unsafe {
            let mut id = 0;
            gl::raw::GenBuffers(1, &mut id);
            gl::raw::BindBuffer(gl::raw::ARRAY_BUFFER, id);
            //     gl::raw::BufferData(
            //         gl::raw::ARRAY_BUFFER,
            //         (VERTICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            //         &VERTICES[0] as *const f32 as *const c_void,
            //         gl::raw::STATIC_DRAW,
            //     );

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

// impl VertexBuffer {
//     pub fn new(draw_kind: DrawKind) -> AppResult<VertexBuffer> {
//         let mut glid = 0;
//         unsafe {
//             gl::raw::GenBuffers(1, &mut glid);
//         }
//         Ok(VertexBuffer { glid, draw_kind })
//     }
// }

// impl GlObject for VertexBuffer {
//     fn as_gl_id(&self) -> GLuint {
//         return self.glid;
//     }
// }
// // let (vbo, vao) = unsafe {
//     let (mut VBO, mut VAO) = (0, 0);
//     gl::raw::GenVertexArrays(1, &mut VAO);
//     gl::raw::GenBuffers(1, &mut VBO);
//     // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
//     gl::raw::BindVertexArray(VAO);

//     gl::raw::BindBuffer(gl::raw::ARRAY_BUFFER, VBO);
//     gl::raw::BufferData(
//         gl::raw::ARRAY_BUFFER,
//         (VERTICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
//         &VERTICES[0] as *const f32 as *const c_void,
//         gl::raw::STATIC_DRAW,
//     );

//     gl::raw::VertexAttribPointer(
//         0,
//         3,
//         gl::raw::FLOAT,
//         gl::raw::FALSE,
//         3 * mem::size_of::<GLfloat>() as GLsizei,
//         ptr::null(),
//     );
//     gl::raw::EnableVertexAttribArray(0);

//     // note that this is allowed, the call to gl::raw::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
//     gl::raw::BindBuffer(gl::raw::ARRAY_BUFFER, 0);

//     // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
//     // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
//     gl::raw::BindVertexArray(0);

//     // uncomment this call to draw in wireframe polygons.
//     // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
//     ( VBO, VAO )
// }
