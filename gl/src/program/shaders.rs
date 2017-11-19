use std::{ffi, ptr};

use errors::GlResult;
use GlObject;
use raw;
use raw::types::*;

pub use program::questions;
pub use program::errors::*;

/// ShaderKind is an typesafe representation of OpenGL shade types.
/// These will map into `raw` shader enum
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ShaderKind {
    /// OpenGL [Vertex Shader](https://www.khronos.org/opengl/wiki/Vertex_Shader)
    Vertex,
    /// OpenGL [Fragment Shader](https://www.khronos.org/opengl/wiki/Fragment_Shader)
    Fragment,
}

impl ShaderKind {
    /// Creates an Example Shader  the given type. This is mostly used for
    /// internal testing, and examples. However, these can be used to bootstrap
    /// quick programs.
    pub fn example(&self) -> &'static str {
        match self {
            &ShaderKind::Vertex => {
                r"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main()
    {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"
            }
            &ShaderKind::Fragment => {
                r"
    #version 330 core
    out vec4 FragColor;
    void main() {
        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"
            }
        }
    }
}

/// # Examples
/// ```
/// use rustlike::{ self,  program, GlObject };
/// use rustlike::raw::types;
///
/// // For Vertex Shader
/// let vertex_kind = program::ShaderKind::Vertex;
/// let vertex: types::GLenum = vertex_kind.into();
/// assert_eq!(raw::VERTEX_SHADER, vertex);
///
/// // For Fragment Shader
/// let fragment_kind = program::ShaderKind::Fragment;
/// let fragment: types::GLenum = fragment_kind.into();
/// assert_eq!(raw::FRAGMENT_SHADER, fragment);
/// ```
impl Into<GLenum> for ShaderKind {
    fn into(self) -> GLenum {
        match self {
            ShaderKind::Vertex => raw::VERTEX_SHADER,
            ShaderKind::Fragment => raw::FRAGMENT_SHADER,
        }
    }
}

/// CompiledShader is an abstraction representation of a
/// compiled [GLSL](https://en.wikipedia.org/wiki/OpenGL_Shading_Language) shader
/// # Examples
/// ```
/// use rustlike::context;
/// use rs_gl;
/// use rs_program::{self, ShaderKind};
/// use rs_raw::types::*;
///
/// let vertex_kind = ShaderKind::Vertex;
/// let ctx = context::ContextBuilder::default().build().unwrap();
/// let vertex_shader = program::CompiledShader::new(vertex_kind.example(), vertex_kind)
///     .unwrap();
/// ```
///
/// # Notes
/// CompiledShader will tell the GPU to destroy itself whenever it is dropped
#[derive(Debug)]
pub struct CompiledShader {
    /// Reference to Shader Allocation in the GPU
    glid: GLuint,
    /// Kind of Shader created
    kind: ShaderKind,
}

impl CompiledShader {
    /// Creates and Compiles Shader.
    pub fn new<T: Into<Vec<u8>>>(src: T, kind: ShaderKind) -> GlResult<CompiledShader> {

        let c_str = ffi::CString::new(src)?;
        unsafe {
            let glid = raw::CreateShader(kind.into());
            raw::ShaderSource(glid, 1, &c_str.as_ptr(), ptr::null());
            raw::CompileShader(glid);

            questions::shader::is_successfully_compiled(glid)?;
            Ok(CompiledShader { glid, kind })
        }
    }

    /// Checks if the CompiledShader is an shader, has been successfully compiled,
    /// and isn't marked for deletion
    pub fn is_valid(&self) -> GlResult<()> {
        let gpu_shader = questions::shader::shader_kind(self.glid)?;
        if gpu_shader != self.kind {
            return Err(
                ProgramError::InvalidShader(format!(
                    "Expected Shader {:?} but gpu things this shader is a {:?}",
                    self.kind,
                    gpu_shader
                )).into(),
            );
        }
        questions::shader::is_deleted(self.glid)?;
        questions::shader::is_successfully_compiled(self.glid)?;
        Ok(())
    }

    /// Returns the kind of shader the Abstraction expects the shader to be
    pub fn kind(&self) -> ShaderKind {
        self.kind
    }
}

impl GlObject for CompiledShader {
    fn as_gl_id(&self) -> GLuint {
        self.glid
    }
}

/// Command OpenGL to delete the Shader. OpenGL won't
/// destroy the shader if it is assigned to a Program so it is
/// safe to Drop the CompiledShader after it has been used for
/// a program. However, OpenGL will clean up the shader after
/// it is cleaned up.
///
/// [Read More](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteShader.xhtml)
impl Drop for CompiledShader {
    fn drop(&mut self) {
        unsafe {
            raw::DeleteShader(self.glid);
        }

    }
}
