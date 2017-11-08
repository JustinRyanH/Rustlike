extern crate sdl2;

use std::time::Duration;
use std::{ptr, mem};
use std::os::raw::c_void;


use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod gl;
pub mod error;
pub mod context;

use gl::GlObject;
use gl::raw::types::*;
use gl::program;
use gl::buffer;
use context::ContextBuilder;
const VERTICES: &[f32] = &[
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


    let builder = buffer::VertexBufferBuilder::new(VERTICES);
    let vao = buffer::VertexArray::new(builder)?;

    'running: loop {
        ctx.present();
        unsafe {
            gl::raw::ClearColor(0.6, 0.0, 0.8, 1.0);
            gl::raw::Clear(gl::raw::COLOR_BUFFER_BIT);
            /// Draw Triangle
            program.set_to_current();
            gl::raw::BindVertexArray(vao.as_gl_id());
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
