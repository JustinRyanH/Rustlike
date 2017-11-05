extern crate sdl2;

use std::time::Duration;
use std::{ffi, ptr, mem};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod gl;
pub mod error;
pub mod context;

use gl::raw::types::*;
use gl::program::compile_shader;
use context::ContextBuilder;

pub fn run() -> error::AppResult<()> {
    let mut ctx = ContextBuilder::default().build()?;

    // TODO: Extract this into a Test
    debug_assert_eq!(ctx.window().subsystem().gl_attr().context_version(), (3, 3));
    let vs = compile_shader(VS_SRC, gl::raw::VERTEX_SHADER);
    let fs = compile_shader(FS_SRC, gl::raw::FRAGMENT_SHADER);

    let program = link_program(vs, fs);

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        // Create Vertex Array Object
        gl::raw::GenVertexArrays(1, &mut vao);
        gl::raw::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::raw::GenBuffers(1, &mut vbo);
        gl::raw::BindBuffer(gl::raw::ARRAY_BUFFER, vbo);
        gl::raw::BufferData(
            gl::raw::ARRAY_BUFFER,
            (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&VERTEX_DATA[0]),
            gl::raw::STATIC_DRAW,
        );

        // Use shader program
        gl::raw::UseProgram(program);
        gl::raw::BindFragDataLocation(program, 0, ffi::CString::new("out_color")?.as_ptr());

        // Specify the layout of the vertex data
        let pos_attr = gl::raw::GetAttribLocation(program, ffi::CString::new("position")?.as_ptr());
        gl::raw::EnableVertexAttribArray(pos_attr as GLuint);
        gl::raw::VertexAttribPointer(
            pos_attr as GLuint,
            2,
            gl::raw::FLOAT,
            gl::raw::FALSE as GLboolean,
            0,
            ptr::null(),
        );
    }

    'running: loop {
        ctx.present();
        unsafe {
            gl::raw::ClearColor(0.6, 0.0, 0.8, 1.0);
            gl::raw::Clear(gl::raw::COLOR_BUFFER_BIT);
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



fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::raw::CreateProgram();
        gl::raw::AttachShader(program, vs);
        gl::raw::AttachShader(program, fs);
        gl::raw::LinkProgram(program);
        // Get the link status
        let mut status = gl::raw::FALSE as GLint;
        gl::raw::GetProgramiv(program, gl::raw::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::raw::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::raw::GetProgramiv(program, gl::raw::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::new();
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::raw::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                String::from_utf8(buf).ok().expect(
                    "ProgramInfoLog not valid utf8",
                )
            );
        }
        program
    }
}
