use std::fmt;
use std::error::Error;

use gl::program::ProgramError;

#[derive(Debug, Clone)]
pub enum GlError {
    ProgramError(ProgramError),
    QuestionError(String),
}

impl fmt::Display for GlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GlError::ProgramError(ref e)  => write!(f, "GlError: {:?}", e),
            GlError::QuestionError(ref s) => write!(f, "QuestionError: {:?}", s),
        }
    }
}

impl Error for GlError {
    fn description(&self) -> &str {
        match *self {
            GlError::ProgramError(_) => "Errors involving shader programs",
            GlError::QuestionError(_) => "Error when asking OpenGL Questions",
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
