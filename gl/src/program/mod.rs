//! Compiled Shader Pipeline
use std::ffi;

pub mod questions;
pub mod errors;
pub mod shaders;
pub mod uniforms;

use GlObject;
use errors::GlResult;
use raw;
use raw::types::*;

pub use program::uniforms::{Uniform, UniformVector, UniformMatrix};
pub use program::errors::ProgramError;
pub use program::shaders::*;

/// ShaderProgram is an abstract representation of
/// [GLSL Object](https://www.khronos.org/opengl/wiki/GLSL_Object)
pub struct ShaderProgram(GLuint);

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

    /// Sets shader Uniform values
    pub unsafe fn update_uniform<T>(&self, name: &str, uniform: T) -> GlResult<()>
    where
        T: Into<Uniform>,
    {
        let c_name = ffi::CString::new(name)?;
        let loc = raw::GetUniformLocation(self.as_gl_id(), c_name.as_ptr());
        match uniform.into() {
            Uniform::ScalarFloat(v) => {
                raw::Uniform1f(loc, v);
            }
            Uniform::ScalarInt(v) => {
                raw::Uniform1i(loc, v);
            }
            Uniform::ScalarUnsignedInt(v) => {
                raw::Uniform1ui(loc, v);
            }
            Uniform::VectorFloat(vec) => {
                match vec {
                    UniformVector::TwoDimensions(x, y) => {
                        raw::Uniform2f(loc, x, y);
                    }
                    UniformVector::ThreeDimensions(x, y, z) => {
                        raw::Uniform3f(loc, x, y, z);
                    }
                    UniformVector::FourDimensions(x, y, z, w) => {
                        raw::Uniform4f(loc, x, y, z, w);
                    }
                }
            }
            Uniform::VectorInt(vec) => {
                match vec {
                    UniformVector::TwoDimensions(x, y) => {
                        raw::Uniform2i(loc, x, y);
                    }
                    UniformVector::ThreeDimensions(x, y, z) => {
                        raw::Uniform3i(loc, x, y, z);
                    }
                    UniformVector::FourDimensions(x, y, z, w) => {
                        raw::Uniform4i(loc, x, y, z, w);
                    }
                }
            }
            Uniform::VectorUnsignedInt(vec) => {
                match vec {
                    UniformVector::TwoDimensions(x, y) => {
                        raw::Uniform2ui(loc, x, y);
                    }
                    UniformVector::ThreeDimensions(x, y, z) => {
                        raw::Uniform3ui(loc, x, y, z);
                    }
                    UniformVector::FourDimensions(x, y, z, w) => {
                        raw::Uniform4ui(loc, x, y, z, w);
                    }
                }
            }
            Uniform::Matrix(matrix) => {
                match matrix {
                    UniformMatrix::Mat2(mat) => {
                        raw::UniformMatrix2fv(loc, 1, raw::FALSE, &mat[0][0]);
                    },
                    _ => unimplemented!(),

                }
            }
        }
        Ok(())
    }
}

impl GlObject for ShaderProgram {
    #[inline]
    fn as_gl_id(&self) -> GLuint {
        self.0
    }
}
