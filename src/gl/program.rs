use std::{ffi, ptr};
use std::fmt;
use std::error::Error;

use error::{AppResult, AppError};
use gl;
use gl::raw::types::*;



/// ShaderKind is an typesafe representation of OpenGL shade types.
/// These will map into `gl::raw` shader enum
#[derive(Debug, Copy, Clone)]
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
/// use rustlike::gl;
/// use rustlike::gl::program;
/// use rustlike::gl::raw::types::*;
///
/// // For Vertex Shader
/// let vertex_kind = program::ShaderKind::Vertex;
/// let vertex: GLenum = vertex_kind.into();
/// assert_eq!(gl::raw::VERTEX_SHADER, vertex);
///
/// // For Fragment Shader
/// let fragment_kind = program::ShaderKind::Fragment;
/// let fragment: GLenum = fragment_kind.into();
/// assert_eq!(gl::raw::FRAGMENT_SHADER, fragment);
/// ```
impl Into<GLenum> for ShaderKind {
    fn into(self) -> GLenum {
        match self {
            ShaderKind::Vertex => gl::raw::VERTEX_SHADER,
            ShaderKind::Fragment => gl::raw::FRAGMENT_SHADER,
        }
    }
}

/// CompiledShader is an abstraction representation of a
/// compiled [GLSL](https://en.wikipedia.org/wiki/OpenGL_Shading_Language) shader
/// # Examples
/// ```
/// use rustlike::context;
/// use rustlike::gl;
/// use rustlike::gl::program::{self, ShaderKind};
/// use rustlike::gl::raw::types::*;
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
    pub fn new<T: Into<Vec<u8>>>(src: T, kind: ShaderKind) -> AppResult<CompiledShader> {

        let c_str = ffi::CString::new(src)?;
        unsafe {
            let glid = gl::raw::CreateShader(kind.into());
            gl::raw::ShaderSource(glid, 1, &c_str.as_ptr(), ptr::null());
            gl::raw::CompileShader(glid);

            // Get the compile status
            let mut status = gl::raw::FALSE as GLint;
            gl::raw::GetShaderiv(glid, gl::raw::COMPILE_STATUS, &mut status);

            if status != (gl::raw::TRUE as GLint) {
                let mut len = 0;
                gl::raw::GetShaderiv(glid, gl::raw::INFO_LOG_LENGTH, &mut len);
                let mut buf: Vec<u8> = Vec::with_capacity(len as usize);
                gl::raw::GetShaderInfoLog(
                    glid,
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
            Ok(CompiledShader { glid, kind })
        }
    }

    /// Returns the OpenGL Id for the compiled shader
    pub fn as_gl_id(&self) -> GLuint {
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
            gl::raw::DeleteShader(self.glid);
        }

    }
}

#[derive(Debug)]
pub enum ProgramError {
    CompilationError(String),
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => write!(f, "ProgramError: {:?}", self),
        }
    }
}

impl Error for ProgramError {
    fn description(&self) -> &str {
        match *self {
            ProgramError::CompilationError(_) => "Error from Compiling Shaders",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
