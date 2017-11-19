use std::{ptr};

use ::errors::{GlResult, GlError};
use ::raw;
use ::raw::types::*;

use ::program::{ProgramError, ShaderKind};

/// Asks Driver questions about the Given Program
pub mod program {
    use super::*;
    #[inline]
    pub fn is_program(id: GLuint) -> GlResult<()> {
        unsafe {
            let result = raw::IsProgram(id) as GLint;
            if result == raw::TRUE as GLint {
                return Ok(());
            } else if result == raw::FALSE as GLint {
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
    pub fn is_linked(id: GLuint) -> GlResult<()> {
        unsafe {
            // Get the link status
            let mut status = raw::FALSE as GLint;
            raw::GetProgramiv(id, raw::LINK_STATUS, &mut status);

            // Fail on error
            if status != (raw::TRUE as GLint) {
                let mut len = 0;
                raw::GetProgramiv(id, raw::INFO_LOG_LENGTH, &mut len);
                let mut buf: Vec<u8> = Vec::with_capacity(len as usize);
                raw::GetProgramInfoLog(
                    id,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );

                buf.set_len(len as usize);
                match String::from_utf8(buf) {
                    Ok(s) => return Err(ProgramError::CompilationError(format!("{}", s)).into()),
                    Err(e) => return Err(GlError::GenericError(format!("{:?}", e))),
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
    /// use rustlike::raw::types::*;
    ///
    /// let vertex_kind = ShaderKind::Vertex;
    /// let ctx = context::ContextBuilder::default().build().unwrap();
    /// let vertex_shader = program::CompiledShader::new(vertex_kind.example(), vertex_kind)
    ///     .unwrap();
    /// assert!(questions::shader::is_shader(vertex_shader.as_gl_id()).is_ok())
    /// ```
    ///
    #[inline]
    pub fn is_shader(id: GLuint) -> GlResult<()> {
        unsafe {
            let result = raw::IsShader(id) as GLint;
            if result == raw::TRUE as GLint {
                return Ok(());
            } else if result == raw::FALSE as GLint {
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
    pub fn is_deleted(id: GLuint) -> GlResult<bool> {
        unsafe {
            is_shader(id)?;
            let mut status = raw::TRUE as GLint;
            raw::GetShaderiv(id, raw::DELETE_STATUS, &mut status);

            if status == raw::FALSE as GLint {
                return Ok(false);
            } else if status == raw::TRUE as GLint {
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
    /// use rustlike::raw::types::*;
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
    pub fn shader_kind(id: GLuint) -> GlResult<ShaderKind> {
        unsafe {
            is_shader(id)?;
            let mut status = 0 as GLint;
            raw::GetShaderiv(id, raw::SHADER_TYPE, &mut status);

            if status == raw::FRAGMENT_SHADER as GLint {
                return Ok(ShaderKind::Fragment);
            } else if status == raw::VERTEX_SHADER as GLint {
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
    pub fn is_successfully_compiled(id: GLuint) -> GlResult<()> {
        unsafe {
            let mut status = raw::FALSE as GLint;
            raw::GetShaderiv(id, raw::COMPILE_STATUS, &mut status);

            if status != (raw::TRUE as GLint) {
                let mut len = 0;
                raw::GetShaderiv(id, raw::INFO_LOG_LENGTH, &mut len);
                let mut buf: Vec<u8> = Vec::with_capacity(len as usize);
                raw::GetShaderInfoLog(
                    id,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );

                buf.set_len(len as usize);

                match String::from_utf8(buf) {
                    Ok(s) => return Err(ProgramError::CompilationError(format!("{}", s)).into()),
                    Err(e) => return Err(GlError::GenericError(format!("{:?}", e))),
                }
            }
            return Ok(());
        }
    }
}
