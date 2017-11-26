//! Errors from interactive with OpenGL Driver
use std::fmt;
use std::ffi;
use std::error::Error;

use program::ProgramError;

/// Results for use of Rustlike GL Shorthand
pub type GlResult<T> = Result<T, GlError>;

/// Error from OpenGL communications
#[derive(Debug, Clone)]
pub enum GlError {
    /// Errors from OpenGL Shader.
    ProgramError(ProgramError),
    /// Error from when invoking Attribute Definitions
    AttributeError(String),
    /// Error from OpenGL Validation
    QuestionError(String),
    /// Error when uncategorized code fails
    GenericError(String),
    /// Error from Operating System Interaction
    SystemError(String),
    /// Error from Binding to Buffers
    BindingError(String),
}

impl fmt::Display for GlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GlError::ProgramError(ref e) => write!(f, "GlError: {:?}", e),
            GlError::QuestionError(ref s) => write!(f, "QuestionError: {:?}", s),
            GlError::AttributeError(ref s) => write!(f, "AttributeError: {:?}", s),
            GlError::GenericError(ref s) => write!(f, "GenericError: {:?}", s),
            GlError::SystemError(ref s) => write!(f, "SystemError: {:?}", s),
            GlError::BindingError(ref s) => write!(f, "BindingError: {:?}", s),
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
            GlError::SystemError(_) => "Errors from interaction with the Operating System.",
            GlError::BindingError(_) => "Errors from interaction binding buffers",
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
