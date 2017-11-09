use std::{ptr};

use error::{AppResult, AppError};
use gl;
use gl::error::GlError;
use gl::raw::types::*;
use gl::program::{ProgramError, ShaderKind};

/// Asks Driver questions about the Given Program
pub mod program {
    use super::*;
    #[inline]
    pub fn is_program(id: GLuint) -> AppResult<()> {
        unsafe {
            let result = gl::raw::IsProgram(id) as GLint;
            if result == gl::raw::TRUE as GLint {
                return Ok(());
            } else if result == gl::raw::FALSE as GLint {
                return Err(
                    GlError::QuestionError(
                        format!("Unknown error during Question `is_program`."),
                    ).into(),
                );
            } else {
                return Err(
                    GlError::QuestionError(format!("{} is not referencing a glProgram", result))
                        .into(),
                );
            }
        }
    }

    #[inline]
    pub fn is_linked(id: GLuint) -> AppResult<()> {
        unsafe {
            // Get the link status
            let mut status = gl::raw::FALSE as GLint;
            gl::raw::GetProgramiv(id, gl::raw::LINK_STATUS, &mut status);

            // Fail on error
            if status != (gl::raw::TRUE as GLint) {
                let mut len = 0;
                gl::raw::GetProgramiv(id, gl::raw::INFO_LOG_LENGTH, &mut len);
                let mut buf: Vec<u8> = Vec::with_capacity(len as usize);
                gl::raw::GetProgramInfoLog(
                    id,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
                buf.set_len(len as usize);
                match String::from_utf8(buf) {
                    Ok(s) => return Err(ProgramError::CompilationError(format!("{}", s)).into()),
                    Err(e) => return Err(AppError::GenericError(format!("{:?}", e))),
                }
            }
            Ok(())
        }
    }
}

/// Asks Driver questions about the Given Shader
pub mod shader {
    use super::*;

    /// Checks if given id is actually an shader, Errors if it is not
    /// # Example
    /// ```
    /// use rustlike::context;
    /// use rustlike::gl;
    /// use rustlike::gl::GlObject;
    /// use rustlike::gl::program::{self, ShaderKind};
    /// use rustlike::gl::program::questions;
    /// use rustlike::gl::raw::types::*;
    ///
    /// let vertex_kind = ShaderKind::Vertex;
    /// let ctx = context::ContextBuilder::default().build().unwrap();
    /// let vertex_shader = program::CompiledShader::new(vertex_kind.example(), vertex_kind)
    ///     .unwrap();
    /// assert!(questions::shader::is_shader(vertex_shader.as_gl_id()).is_ok())
    /// ```
    ///
    #[inline]
    pub fn is_shader(id: GLuint) -> AppResult<()> {
        unsafe {
            let result = gl::raw::IsShader(id) as GLint;
            if result == gl::raw::TRUE as GLint {
                return Ok(());
            } else if result == gl::raw::FALSE as GLint {
                return Err(
                    GlError::QuestionError(
                        format!("Unknown error during Question `is_shader`."),
                    ).into(),
                );
            } else {
                return Err(
                    GlError::QuestionError(format!("{} is not referencing a glShader", result))
                        .into(),
                );
            }
        }
    }

    /// Checks if Driver is marked for Deletion, Errors if it is not a shader
    /// TODO: Example
    #[inline]
    pub fn is_deleted(id: GLuint) -> AppResult<bool> {
        unsafe {
            is_shader(id)?;
            let mut status = gl::raw::TRUE as GLint;
            gl::raw::GetShaderiv(id, gl::raw::DELETE_STATUS, &mut status);

            if status == gl::raw::FALSE as GLint {
                return Ok(false);
            } else if status == gl::raw::TRUE as GLint {
                return Ok(true);
            }
            return Err(
                GlError::QuestionError(format!(
                    "OpenGL Don Fucked up. Expect raw::TRUE or raw::FALSE, but got: {}",
                    status
                )).into(),
            );
        }
    }


    /// Returns kind of Shader in local abstraction, Errors if it is not a shader
    /// # Example
    /// ```
    /// use rustlike::context;
    /// use rustlike::gl;
    /// use rustlike::gl::GlObject;
    /// use rustlike::gl::program::{self, ShaderKind};
    /// use rustlike::gl::program::questions;
    /// use rustlike::gl::raw::types::*;
    ///
    /// let fragment_kind = ShaderKind::Fragment;
    /// let ctx = context::ContextBuilder::default().build().unwrap();
    /// let fragment_shader = program::CompiledShader::new(fragment_kind.example(), fragment_kind)
    ///     .unwrap();
    /// assert_eq!(
    ///     questions::shader::shader_kind(fragment_shader.as_gl_id()).unwrap(),
    ///     fragment_kind
    /// )
    /// ```
    ///
    #[inline]
    pub fn shader_kind(id: GLuint) -> AppResult<ShaderKind> {
        unsafe {
            is_shader(id)?;
            let mut status = 0 as GLint;
            gl::raw::GetShaderiv(id, gl::raw::SHADER_TYPE, &mut status);

            if status == gl::raw::FRAGMENT_SHADER as GLint {
                return Ok(ShaderKind::Fragment);
            } else if status == gl::raw::VERTEX_SHADER as GLint {
                return Ok(ShaderKind::Vertex);
            }
            return Err(
                GlError::QuestionError(format!(
                    "Expected Shader Type, from driver, but got: {}",
                    status
                )).into(),
            );
        }
    }

    #[inline]
    pub fn is_successfully_compiled(id: GLuint) -> AppResult<()> {
        unsafe {
            let mut status = gl::raw::FALSE as GLint;
            gl::raw::GetShaderiv(id, gl::raw::COMPILE_STATUS, &mut status);

            if status != (gl::raw::TRUE as GLint) {
                let mut len = 0;
                gl::raw::GetShaderiv(id, gl::raw::INFO_LOG_LENGTH, &mut len);
                let mut buf: Vec<u8> = Vec::with_capacity(len as usize);
                gl::raw::GetShaderInfoLog(
                    id,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );

                buf.set_len(len as usize);

                match String::from_utf8(buf) {
                    Ok(s) => return Err(ProgramError::CompilationError(format!("{}", s)).into()),
                    Err(e) => return Err(AppError::GenericError(format!("{:?}", e))),
                }
            }
            return Ok(());
        }
    }
}
