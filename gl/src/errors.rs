use std::fmt;
use std::ffi;
use std::error::Error;

use program::ProgramError;

pub type GlResult<T> = Result<T, GlError>;

#[derive(Debug, Clone)]
pub enum GlError {
    ProgramError(ProgramError),
    AttributeError(String),
    QuestionError(String),
    GenericError(String),
    SystemError(String),
}

impl fmt::Display for GlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GlError::ProgramError(ref e)  => write!(f, "GlError: {:?}", e),
            GlError::QuestionError(ref s) => write!(f, "QuestionError: {:?}", s),
            GlError::AttributeError(ref s) => write!(f, "AttributeError: {:?}", s),
            GlError::GenericError(ref s) => write!(f, "GenericError: {:?}",s ),
            GlError::SystemError(ref s) => write!(f, "SystemError: {:?}",s ),
        }
    }
}

impl Error for GlError {
    fn description(&self) -> &str {
        match *self {
            GlError::ProgramError(_) => "Errors involving shader programs",
            GlError::AttributeError(_) => "Errors involving attribute definitions",
            GlError::QuestionError(_) => "Error when asking OpenGL Questions",
            GlError::GenericError(_) => "Errors when uncategorized code fails.",
            GlError::SystemError(_) => "Errors from interaction with the Operating System."
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            _ => None,
        }
    }
}

impl From<ProgramError> for GlError {
    fn from(e: ProgramError) -> GlError {
        GlError::ProgramError(e)
    }
}

impl From<ffi::NulError> for GlError {
    fn from(e: ffi::NulError) -> GlError {
        GlError::SystemError(format!("NulError: {:?}", e))
    }
}

