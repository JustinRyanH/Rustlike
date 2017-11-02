use std::default::Default;

use sdl2::VideoSubsystem;
use sdl2::video::{Window, WindowBuilder};

use error::{AppResult, AppError};


pub enum Screensize {
    Fullscreen,
    Resizable,
    None,
}


pub enum Graphics {
    OpenGL,
    Vulkan,
}

impl Graphics {
    #[cfg(any(windows, linux))]
    pub fn set_graphics(self, builder: &mut WindowBuilder) -> AppResult<()> {
        match self {
            Graphics::OpenGL => builder.opengl(),
            Graphics::Vulkan => builder.vulkan(),
        };
        Ok(())
    }

    #[cfg(not(any(windows, linux)))]
    pub fn set_graphics(self, builder: &mut WindowBuilder) -> AppResult<()> {
        match self {
            Graphics::OpenGL => builder.opengl(),
            Graphics::Vulkan => {
                return Err(AppError::WindowError(String::from(
                    "Sorry, Vulkan is not supported on your operating system",
                )))
            }
        };
        Ok(())
    }
}

pub enum Position {
    Centered,
    Position(i32, i32),
}

pub struct ContextBuilder {
    pub borderless: bool,
    pub screensize: Screensize,
    pub graphics: Graphics,
    pub position: Position,
    pub size: (u32, u32),
    pub title: String,
}

impl ContextBuilder {
    pub fn build(self, video_subsystem: &VideoSubsystem) -> AppResult<Window> {
        let mut builder = WindowBuilder::new(
            &video_subsystem,
            self.title.as_str(),
            self.size.0,
            self.size.1,
        );

        match self.position {
            Position::Centered => builder.position_centered(),
            Position::Position(x, y) => builder.position(x, y),
        };
        self.graphics.set_graphics(&mut builder)?;
        if self.borderless {
            builder.borderless();
        }

        match self.screensize {
            Screensize::Resizable => builder.resizable(),
            Screensize::Fullscreen => builder.fullscreen(),
            Screensize::None => &mut builder,
        };

        Ok(builder.build()?)
    }
}


impl Default for ContextBuilder {
    fn default() -> ContextBuilder {
        ContextBuilder {
            borderless: false,
            screensize: Screensize::Resizable,
            graphics: Graphics::OpenGL,
            position: Position::Centered,
            size: (300, 300),
            title: "Foobar".to_owned(),
        }
    }
}
