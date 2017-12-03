#[macro_use]
extern crate rl_gl_derive;
extern crate rl_gl;

extern crate sdl2;

use std::collections::HashSet;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use rl_gl::buffer::BufferBuilder;
use rl_gl::program::uniforms::NamedUniform;
use rl_gl::UpdatableUniforms;

pub mod errors;
pub mod context;


#[derive(Clone, DescribeAttributes)]
struct ExampleData {
    pos: [f32; 3],
}

#[derive(Clone, UpdatableUniforms)]
struct ExampleUniform {
    out_color: [f32; 4],
    to_update: HashSet<&'static str>
}

impl ExampleUniform {
    pub fn new(color: [f32; 4]) -> ExampleUniform {
        ExampleUniform{
            out_color: color,
            to_update: HashSet::new(),
        }
    }

    #[inline]
    pub fn update_color(&mut self, color: [f32; 4]) -> &mut ExampleUniform {
        self.out_color = color;
        self.to_update.insert("out_color");
        self
    }
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

    let mut uniforms = ExampleUniform::new([0., 1., 0., 1.]);
    unsafe {
        program.set_to_current();
        program.set_uniform_values(uniforms.uniform_values())?;
    }

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
            uniforms.update_color([0., green_value, 0., 1.]);
            program.update_uniform_values(&mut uniforms)?;
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
out vec4 outColor;
uniform vec4 out_color;
void main() {
    outColor = out_color;
}"#;
