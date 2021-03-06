//! Program Errors

use std::fmt;
use std::error::Error;

/// Errors Involving Shaders
#[derive(Debug, Clone)]
pub enum ProgramError {
    /// Error when compiling Shader
    CompilationError(String),
    /// Error when shader changes from expectation
    InvalidShader(String),
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ProgramError::CompilationError(ref s) => write!(f, "CompilationError: {:?}", s),
            &ProgramError::InvalidShader(ref s) => write!(f, "InvalidShader: {:?}", s),
        }
    }
}

impl Error for ProgramError {
    fn description(&self) -> &str {
        match *self {
            ProgramError::CompilationError(_) => "Error from Compiling Shaders",
            ProgramError::InvalidShader(_) => "Error when shader state changes from expectation",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
