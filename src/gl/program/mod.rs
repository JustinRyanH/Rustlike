pub mod questions;
pub mod errors;
pub mod shaders;
// mod programs;

use error::{AppResult};
use gl;
use gl::GlObject;
use gl::raw::types::*;

pub use gl::program::errors::ProgramError;
pub use gl::program::shaders::*;

pub struct ShaderProgram(GLuint);

/// ShaderProgram is an abstract representation of [GLSL Object](https://www.khronos.org/opengl/wiki/GLSL_Object)
impl ShaderProgram {
    /// Creates a Shader Program by linking the given Vertex Shader and the Fragment Shader
    ///
    /// # Example
    /// ```
    /// use rustlike::context;
    /// use rustlike::gl::GlObject;
    /// use rustlike::gl::program::{self, ShaderKind, questions};
    ///
    /// let vertex_kind = ShaderKind::Vertex;
    /// let fragment_kind = ShaderKind::Fragment;
    /// let _ctx = context::ContextBuilder::default().build().unwrap();
    /// let vertex_shader = program::CompiledShader::new(vertex_kind.example(), vertex_kind)
    ///     .unwrap();
    /// let fragment_shader = program::CompiledShader::new(fragment_kind.example(), fragment_kind)
    ///     .unwrap();
    /// let program = program::ShaderProgram::new(&vertex_shader, &fragment_shader).unwrap();
    /// assert!(questions::program::is_program(program.as_gl_id()).is_ok());
    /// ```
    pub fn new(
        vertex_shader: &CompiledShader,
        fragment_shader: &CompiledShader,
    ) -> AppResult<ShaderProgram> {
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
            let program = gl::raw::CreateProgram();
            gl::raw::AttachShader(program, vertex_shader.as_gl_id());
            gl::raw::AttachShader(program, fragment_shader.as_gl_id());
            gl::raw::LinkProgram(program);
            questions::program::is_linked(program)?;
            program
        };

        Ok(ShaderProgram(program))
    }

    /// TODO: Test with Example
    pub fn set_to_current(&self) {
        unsafe {
            gl::raw::UseProgram(self.0);
        }
    }
}

impl gl::GlObject for ShaderProgram {
    #[inline]
    fn as_gl_id(&self) -> GLuint {
        self.0
    }
}
