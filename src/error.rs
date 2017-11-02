use std::fmt;
use std::error::Error;

use sdl2;

#[derive(Debug)]
pub enum AppError {
    WindowError(String),
    GenericError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &AppError::WindowError(ref err) => write!(f, "WindowError: {:}", err),
            &AppError::GenericError(ref err) => write!(f, "GenericError: {:}", err),
        }
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        match *self {
            AppError::WindowError(_) => "Errors involving Window context",
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
pub type AppResult<T> = Result<T, AppError>;
