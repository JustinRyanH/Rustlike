use std::fmt;
use std::error::Error;
use std::{ptr, mem};
use std::ffi::CString;
use std::str;


use gl::raw;
use gl::raw::types::*;

use error::AppResult;

pub struct ShaderProgram {
    gl_id: GLuint,
    shader_type: ShaderType,
}

impl ShaderProgram {
    // #TODO: Pass in Context which confirms the Gl version
    pub fn new(shader_type: ShaderType, source: &str) -> AppResult<ShaderProgram> {
        let gl_id = unsafe { raw::CreateShader(shader_type.into()) };
        let program = ShaderProgram { gl_id, shader_type };
        Ok(program)

    }
}

impl<'a> From<&'a ShaderProgram> for Shader<'a> {
    fn from(program: &'a ShaderProgram) -> Shader<'a> {
        match program.shader_type {
            ShaderType::Vertex => Shader::Vertex(program),
            ShaderType::Fragment => Shader::Fragment(program),
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            raw::DeleteShader(self.gl_id);
        }
    }
}

#[derive(Clone, Copy)]
pub enum ShaderType {
    Vertex,
    Fragment,
}

impl Into<GLenum> for ShaderType {
    fn into(self) -> GLenum {
        match self {
            ShaderType::Vertex => raw::VERTEX_SHADER,
            ShaderType::Fragment => raw::FRAGMENT_SHADER,
        }
    }
}

pub enum Shader<'a> {
    Vertex(&'a ShaderProgram),
    Fragment(&'a ShaderProgram),
}

#[derive(Debug)]
pub enum ProgramError {
    CompileError(String),
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ProgramError::CompileError(ref err) => write!(f, "CompileError:\n\t {:}", err),
        }
    }
}

impl Error for ProgramError {
    fn description(&self) -> &str {
        match *self {
            ProgramError::CompileError(_) => "Errors from Compiling Shaders Errors",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
