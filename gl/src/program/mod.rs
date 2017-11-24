//! Compiled Shader Pipeline
pub mod questions;
pub mod errors;
pub mod shaders;

use GlObject;
use errors::GlResult;
use raw;
use raw::types::*;

pub use program::errors::ProgramError;
pub use program::shaders::*;

pub struct ShaderProgram(GLuint);

/// ShaderProgram is an abstract representation of [GLSL Object](https://www.khronos.org/opengl/wiki/GLSL_Object)
impl ShaderProgram {
    /// Creates a Shader Program by linking the given Vertex Shader and the Fragment Shader
    pub fn new(
        vertex_shader: &CompiledShader,
        fragment_shader: &CompiledShader,
    ) -> GlResult<ShaderProgram> {
        if vertex_shader.kind() != ShaderKind::Vertex {
            return Err(
                ProgramError::InvalidShader(format!(
                    "Expected Vertex Shader, but got a {:?} Shader",
                    vertex_shader.kind()
                )).into(),
            );
        }
        if fragment_shader.kind() != ShaderKind::Fragment {
            return Err(
                ProgramError::InvalidShader(format!(
                    "Expected Fragment Shader, but got a {:?} Shader",
                    vertex_shader.kind()
                )).into(),
            );
        }

        vertex_shader.is_valid()?;
        fragment_shader.is_valid()?;

        let program = unsafe {
            let program = raw::CreateProgram();
            raw::AttachShader(program, vertex_shader.as_gl_id());
            raw::AttachShader(program, fragment_shader.as_gl_id());
            raw::LinkProgram(program);
            questions::program::is_linked(program)?;
            program
        };

        Ok(ShaderProgram(program))
    }

    /// Sets the program to the current program in the context
    pub fn set_to_current(&self) {
        unsafe {
            raw::UseProgram(self.0);
        }
    }
}

impl GlObject for ShaderProgram {
    #[inline]
    fn as_gl_id(&self) -> GLuint {
        self.0
    }
}
