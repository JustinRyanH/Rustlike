extern crate sdl2;

use std::time::Duration;
use std::{ffi, ptr, mem};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod gl;
pub mod error;
pub mod builder;

use error::AppResult;
use gl::raw::types::*;

struct Context {
    sdl: sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    canvas: sdl2::render::WindowCanvas,
    event_pump: sdl2::EventPump,
}

impl Context {
    // Initializes Window, Events, and Graphics Contexts
    pub fn new() -> AppResult<Context> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;

        // TODO: This should run inside the Builder's `build()`
        {
            let ref gl_attributes = video.gl_attr();
            gl_attributes.set_context_major_version(3);
            gl_attributes.set_context_minor_version(3);
            gl_attributes.set_context_profile(sdl2::video::GLProfile::Core);
            gl_attributes.set_depth_size(24);
            gl_attributes.set_double_buffer(true);
            video.gl_set_swap_interval(1);
        }

        // TODO: In house builder should just return this guy
        let canvas = video
            .window("Window", 800, 600)
            .opengl()
            .build()?
            .into_canvas()
            .index(Context::find_render_driver(Drivers::OpenGL).unwrap())
            .build()?;

        let event_pump = sdl.event_pump()?;

        Ok(Context {
            sdl,
            video,
            canvas,
            event_pump,
        })
    }

    /// Finds the render driver passed into method unless it doesn't exists
    pub fn find_render_driver(driver: Drivers) -> Option<u32> {
        let d_str: &str = driver.into();
        for (index, item) in sdl2::render::drivers().enumerate() {
            if item.name == d_str {
                return Some(index as u32);
            }
        }
        None
    }

    pub fn window(&self) -> &sdl2::video::Window {
        self.canvas.window()
    }

    pub fn present(&mut self) {
        self.canvas.present()
    }

    pub fn poll_iter(&mut self) -> sdl2::event::EventPollIterator {
        self.event_pump.poll_iter()
    }
}



enum Drivers {
    OpenGL,
}

impl<'a> Into<&'a str> for Drivers {
    fn into(self) -> &'a str {
        match self {
            Drivers::OpenGL => "opengl",
        }
    }
}

pub fn run() -> error::AppResult<()> {
    let mut ctx = Context::new()?;

    gl::raw::load_with(|name| ctx.video.gl_get_proc_address(name) as *const _);
    ctx.window().gl_set_context_to_current();
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

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::raw::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = ffi::CString::new(src.as_bytes()).unwrap();
        gl::raw::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::raw::CompileShader(shader);

        // Get the compile status
        let mut status = gl::raw::FALSE as GLint;
        gl::raw::GetShaderiv(shader, gl::raw::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::raw::TRUE as GLint) {
            let mut len = 0;
            gl::raw::GetShaderiv(shader, gl::raw::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::new();
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::raw::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                String::from_utf8(buf).ok().expect(
                    "ShaderInfoLog not valid utf8",
                )
            );
        }
    }
    shader
}

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
