use std::fmt;
use std::error::Error;

use gl::shader::ProgramError;

#[derive(Debug)]
pub enum GlError {
    ProgramError(ProgramError),
}

impl fmt::Display for GlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => write!(f, "GlError: {:?}", self),
        }
    }
}

impl Error for GlError {
    fn description(&self) -> &str {
        match *self {
            GlError::ProgramError(_) => "Errors involving Compiling and Linking Shaders",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl From<ProgramError> for GlError {
    fn from(e: ProgramError) -> GlError {
        GlError::ProgramError(e)
    }
}
