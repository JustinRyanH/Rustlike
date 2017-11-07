extern crate sdl2;

use std::time::Duration;
use std::{ffi, ptr, mem};
use std::os::raw::c_void;


use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod gl;
pub mod error;
pub mod context;

use gl::GlObject;
use gl::raw::types::*;
use gl::program;
use context::ContextBuilder;
const VERTICES: [f32; 9] = [
    -0.5, -0.5, 0.0, // left
    0.5, -0.5, 0.0, // right
    0.0,  0.5, 0.0  // top
];

pub fn run() -> error::AppResult<()> {
    let mut ctx = ContextBuilder::default().build()?;

    // TODO: Extract this into a Test
    debug_assert_eq!(ctx.window().subsystem().gl_attr().context_version(), (3, 3));
    let vs_id: GLuint;
    let program = {
        let vs = program::CompiledShader::new(VS_SRC, program::ShaderKind::Vertex)?;
        let fs = program::CompiledShader::new(FS_SRC, program::ShaderKind::Fragment)?;
        vs_id = vs.as_gl_id();
        program::ShaderProgram::new(&vs, &fs)?
    };
    // TODO: Move this into a spec
    debug_assert!(program::questions::shader::is_shader(vs_id).is_ok());
    // TODO: Move this into a spec
    debug_assert_eq!(program::questions::shader::is_deleted(vs_id).unwrap(), true);

    let (vbo, vao) = unsafe {
        let (mut VBO, mut VAO) = (0, 0);
        gl::raw::GenVertexArrays(1, &mut VAO);
        gl::raw::GenBuffers(1, &mut VBO);
        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::raw::BindVertexArray(VAO);

        gl::raw::BindBuffer(gl::raw::ARRAY_BUFFER, VBO);
        gl::raw::BufferData(
            gl::raw::ARRAY_BUFFER,
            (VERTICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &VERTICES[0] as *const f32 as *const c_void,
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

        // note that this is allowed, the call to gl::raw::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::raw::BindBuffer(gl::raw::ARRAY_BUFFER, 0);

        // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl::raw::BindVertexArray(0);

        // uncomment this call to draw in wireframe polygons.
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        ( VBO, VAO )
    };


    'running: loop {
        ctx.present();
        unsafe {
            gl::raw::ClearColor(0.6, 0.0, 0.8, 1.0);
            gl::raw::Clear(gl::raw::COLOR_BUFFER_BIT);
            /// Draw Triangle
            program.set_to_current();
            gl::raw::BindVertexArray(vao);
            gl::raw::DrawArrays(gl::raw::TRIANGLES, 0, 3);
        }
        for event in ctx.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}

// Vertex data
static VERTEX_DATA: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];

// Shader sources
static VS_SRC: &'static str = "#version 150\n\
    in vec2 position;\n\
    void main() {\n\
    gl_Position = vec4(position, 0.0, 1.0);\n\
    }";

static FS_SRC: &'static str = "#version 150\n\
    out vec4 out_color;\n\
    void main() {\n\
       out_color = vec4(1.0, 1.0, 1.0, 1.0);\n\
       }";
