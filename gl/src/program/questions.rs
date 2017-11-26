//! Program OpenGL validity checks

use std::ptr;


use errors::{GlResult, GlError};
use raw::types::*;
use raw;

use program::{ProgramError, ShaderKind};

/// Asks Driver questions about the Given Program
pub mod program {
    use super::*;
    /// Checks if GlObject in question is an program
    /// # Errors
    /// when GlObject is not an Program
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

    /// Checks if GlObject in question is property linked
    /// # Errors
    /// when GlObject is not an Program,
    ///
    /// when GlObject is not linked
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
                raw::GetProgramInfoLog(id, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);

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

    /// Checks if GlObject is actually an shader
    /// # Errors
    /// when not an shader
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

    /// Checks if Shader is marked for Deletion.
    /// # Returns
    /// `true` if deleted
    ///
    /// `false` if still valid
    /// # Errors
    /// when not a shader
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


    /// Returns kind of Shader in local abstraction
    /// # Errors
    /// when not an shader,
    ///
    /// when shader kind is not defined in rl_gl
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

    /// Checks if the Shader has Succefully compiled
    /// # Errors
    /// when not an shader,
    /// when shader has failed to compiled
    #[inline]
    pub fn is_successfully_compiled(id: GLuint) -> GlResult<()> {
        unsafe {
            is_shader(id)?;
            let mut status = raw::FALSE as GLint;
            raw::GetShaderiv(id, raw::COMPILE_STATUS, &mut status);

            if status != (raw::TRUE as GLint) {
                let mut len = 0;
                raw::GetShaderiv(id, raw::INFO_LOG_LENGTH, &mut len);
                let mut buf: Vec<u8> = Vec::with_capacity(len as usize);
                raw::GetShaderInfoLog(id, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);

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
