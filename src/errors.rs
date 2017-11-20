use std::fmt;
use std::ffi;
use std::error::Error;

use sdl2;
use rl_gl;

#[derive(Debug)]
pub enum AppError {
    WindowError(String),
    GfxError(rl_gl::errors::GlError),
    GenericError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &AppError::WindowError(ref err) => write!(f, "WindowError: {:}", err),
            &AppError::GfxError(ref err) => write!(f, "GfxError: {:}", err),
            &AppError::GenericError(ref err) => write!(f, "GenericError: {:}", err),
        }
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        match *self {
            AppError::WindowError(_) => "Errors involving Window context",
            AppError::GfxError(_) => "Errors involving communication with the graphics driver",
            AppError::GenericError(_) => "Unspecified Errors",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl From<String> for AppError {
    fn from(v: String) -> AppError {
        AppError::GenericError(v)
    }
}

impl From<sdl2::video::WindowBuildError> for AppError {
    fn from(v: sdl2::video::WindowBuildError) -> AppError {
        AppError::WindowError(format!("BuildError: {:?}", v))
    }
}
impl From<sdl2::IntegerOrSdlError> for AppError {
    fn from(v: sdl2::IntegerOrSdlError) -> AppError {
        AppError::WindowError(format!("SDLError: {:?}", v))
    }
}

impl From<rl_gl::errors::GlError> for AppError
{
    fn from(v: rl_gl::errors::GlError) -> AppError {
        AppError::GfxError(v)
    }
}

impl From<ffi::NulError> for AppError {
    fn from(v: ffi::NulError) -> AppError {
        AppError::GenericError(format!("C_String Conversion: {:?}", v))
    }
}

pub type AppResult<T> = Result<T, AppError>;
