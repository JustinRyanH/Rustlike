#[macro_use]
extern crate rl_gl_derive;
extern crate rl_gl;

extern crate sdl2;

use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use rl_gl::buffer::BufferBuilder;

pub mod errors;
pub mod context;


#[derive(Clone, DescribeAttributes)]
struct ExampleData {
    pos: [f32; 3],
}

use rl_gl::{program, DescribeAttributes, Attribute};
use context::ContextBuilder;

pub fn run() -> errors::AppResult<()> {
    let epoch = Instant::now();
    let mut ctx = ContextBuilder::default().build()?;

    // TODO: Extract this into a Test
    debug_assert_eq!(ctx.window().subsystem().gl_attr().context_version(), (3, 3));
    let program = {
        let vs = program::CompiledShader::new(VS_SRC, program::ShaderKind::Vertex)?;
        let fs = program::CompiledShader::new(FS_SRC, program::ShaderKind::Fragment)?;
        program::ShaderProgram::new(&vs, &fs)?
    };

    let vertices = vec![
        ExampleData { pos: [0.5, 0.5, 0.0] },
        ExampleData { pos: [0.5, -0.5, 0.0] },
        ExampleData { pos: [-0.5, -0.5, 0.0] },
        ExampleData { pos: [-0.5, 0.5, 0.0] },
    ];

    let indices = vec![0, 1, 3, 1, 2, 3];

    let gl_obj = rl_gl::buffer::BufferConfiguration::new(vertices)
        .with_index(indices)
        .build()?;

    let mut elapse: Duration;
    'running: loop {
        ctx.present();
        unsafe {
            rl_gl::raw::ClearColor(0.6, 0.0, 0.8, 1.0);
            rl_gl::raw::Clear(rl_gl::raw::COLOR_BUFFER_BIT);
            program.set_to_current();
            elapse = epoch.elapsed();
            let since = elapse.as_secs() as f32 + elapse.subsec_nanos() as f32 * 1e-9;
            let green_value = since.sin() / 2. + 0.5;
            program.update_uniform("outColor", [0., green_value, 0., 1.])?;
            gl_obj.draw()?;
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
static VS_SRC: &'static str = r#"
#version 150
in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}"#;

static FS_SRC: &'static str = r#"
#version 150
out vec4 out_color;
uniform vec4 outColor;
void main() {
    out_color = outColor;
}"#;
