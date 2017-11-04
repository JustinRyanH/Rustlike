extern crate sdl2;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod gl;
pub mod error;
pub mod builder;
pub mod context;

static VS_SRC: &'static str =
    "#version 150\n\
     in vec3 position;\n\
     void main() {\n\
     gl_Position = vec4(position, 1.0);\n\
     }";

pub fn run() -> error::AppResult<()> {
    let mut context = context::Context::new(Default::default())?;

    let program = gl::shader::ShaderProgram::new(gl::shader::ShaderType::Vertex, vertexShaderSource)?;

    'running: loop {
        context.blip();
        for event in context.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
