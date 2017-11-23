#[macro_use]
extern crate rl_gl_derive;
extern crate rl_gl;

extern crate sdl2;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod errors;
pub mod context;

#[derive(Clone, DescribeAttributes)]
struct ExampleData {
    pos: [f32; 3],
}

use rl_gl::{program, buffer, DescribeAttributes, Attribute};
use context::ContextBuilder;

pub fn run() -> errors::AppResult<()> {
    let mut ctx = ContextBuilder::default().build()?;

    // TODO: Extract this into a Test
    debug_assert_eq!(ctx.window().subsystem().gl_attr().context_version(), (3, 3));
    let program = {
        let vs = program::CompiledShader::new(VS_SRC, program::ShaderKind::Vertex)?;
        let fs = program::CompiledShader::new(FS_SRC, program::ShaderKind::Fragment)?;
        program::ShaderProgram::new(&vs, &fs)?
    };


    let vertices = vec![
        ExampleData {
            pos: [-0.5, -0.5, 0.0],
        },
        ExampleData {
            pos: [0.5, -0.5, 0.0],
        },
        ExampleData {
            pos: [0.0, 0.5, 0.0],
        },
    ];
    let rl_gl_object = buffer::BufferConfiguration::new(vertices).build()?;
    'running: loop {
        ctx.present();
        unsafe {
            rl_gl::raw::ClearColor(0.6, 0.0, 0.8, 1.0);
            rl_gl::raw::Clear(rl_gl::raw::COLOR_BUFFER_BIT);
            program.set_to_current();
            rl_gl_object.draw()?;
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
