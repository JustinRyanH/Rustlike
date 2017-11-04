extern crate sdl2;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod gl;
pub mod error;
pub mod builder;

static VS_SRC: &'static str =
    "#version 150\n\
     in vec3 position;\n\
     void main() {\n\
     gl_Position = vec4(position, 1.0);\n\
     }";

struct Context {
    
}

impl Context {
    /// Iterates over the active SDL drivers and finds one used
    /// for OpenGL communication
    pub fn find_sdl_gl_driver() -> Option<u32> {
        for (index, item) in sdl2::render::drivers().enumerate() {
            if item.name == "opengl" {
                return Some(index as u32);
            }
        }
        None
    }
}


pub fn run() -> error::AppResult<()> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Window", 800, 600)
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas()
        .index(Context::find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();

    gl::raw::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    canvas.window().gl_set_context_to_current();


    let mut event_pump = sdl_context.event_pump().unwrap();


    // let mut context = context::Context::new(Default::default())?;
    'running: loop {
    //     context.blip();
        unsafe {
            gl::raw::ClearColor(0.6, 0.0, 0.8, 1.0);
            gl::raw::Clear(gl::raw::COLOR_BUFFER_BIT);
        }

        canvas.present();
        for event in event_pump.poll_iter() {
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
