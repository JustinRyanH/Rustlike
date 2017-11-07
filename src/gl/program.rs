use std::{ffi, ptr};
use std::fmt;
use std::error::Error;

use error::{AppResult, AppError};
use gl;
use gl::GlObject;
use gl::error::GlError;
use gl::raw::types::*;


/// Asks Driver questions about the Given Program
pub mod program_questions {
    use super::*;
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
                gl::raw::GetProgramInfoLog(id, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
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
pub mod shader_questions {
    use super::*;

    /// Checks if given id is actually an shader, Errors if it is not
    /// # Example
    /// ```
    /// use rustlike::context;
    /// use rustlike::gl;
    /// use rustlike::gl::GlObject;
    /// use rustlike::gl::program::{self, ShaderKind};
    /// use rustlike::gl::program::shader_questions;
    /// use rustlike::gl::raw::types::*;
    ///
    /// let vertex_kind = ShaderKind::Vertex;
    /// let ctx = context::ContextBuilder::default().build().unwrap();
    /// let vertex_shader = program::CompiledShader::new(vertex_kind.example(), vertex_kind)
    ///     .unwrap();
    /// assert!(shader_questions::is_shader(vertex_shader.as_gl_id()).is_ok())
    /// ```
    ///
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
    /// use rustlike::gl::program::shader_questions;
    /// use rustlike::gl::raw::types::*;
    ///
    /// let fragment_kind = ShaderKind::Fragment;
    /// let ctx = context::ContextBuilder::default().build().unwrap();
    /// let fragment_shader = program::CompiledShader::new(fragment_kind.example(), fragment_kind)
    ///     .unwrap();
    /// assert_eq!(
    ///     shader_questions::shader_kind(fragment_shader.as_gl_id()).unwrap(),
    ///     fragment_kind
    /// )
    /// ```
    ///
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

/// ShaderKind is an typesafe representation of OpenGL shade types.
/// These will map into `gl::raw` shader enum
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
/// use rustlike::gl::{ self,  program, GlObject };
/// use rustlike::gl::raw::types;
///
/// // For Vertex Shader
/// let vertex_kind = program::ShaderKind::Vertex;
/// let vertex: types::GLenum = vertex_kind.into();
/// assert_eq!(gl::raw::VERTEX_SHADER, vertex);
///
/// // For Fragment Shader
/// let fragment_kind = program::ShaderKind::Fragment;
/// let fragment: types::GLenum = fragment_kind.into();
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

            shader_questions::is_successfully_compiled(glid)?;
            Ok(CompiledShader { glid, kind })
        }
    }

    /// Checks if the CompiledShader is an shader, has been successfully compiled,
    /// and isn't marked for deletion
    pub fn is_valid(&self) -> AppResult<()> {
        let gpu_shader = shader_questions::shader_kind(self.glid)?;
        if gpu_shader != self.kind {
            return Err(
                ProgramError::InvalidShader(format!(
                    "Expected Shader {:?} but gpu things this shader is a {:?}",
                    self.kind,
                    gpu_shader
                )).into(),
            );
        }
        shader_questions::is_deleted(self.glid)?;
        shader_questions::is_successfully_compiled(self.glid)?;
        Ok(())
    }

    /// Returns the kind of shader the Abstraction expects the shader to be
    pub fn kind(&self) -> ShaderKind {
        self.kind
    }
}

impl gl::GlObject for CompiledShader {
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
            gl::raw::DeleteShader(self.glid);
        }

    }
}
pub struct ShaderProgram(GLuint);

/// ShaderProgram is an abstract representation of [GLSL Object](https://www.khronos.org/opengl/wiki/GLSL_Object)
impl ShaderProgram {
    /// Creates a Shader Program by linking the given Vertex Shader and the Fragment Shader
    ///
    /// # Example
    /// ```
    /// use rustlike::context;
    /// use rustlike::gl::GlObject;
    /// use rustlike::gl::program::{self, ShaderKind, program_questions};
    ///
    /// let vertex_kind = ShaderKind::Vertex;
    /// let fragment_kind = ShaderKind::Fragment;
    /// let _ctx = context::ContextBuilder::default().build().unwrap();
    /// let vertex_shader = program::CompiledShader::new(vertex_kind.example(), vertex_kind)
    ///     .unwrap();
    /// let fragment_shader = program::CompiledShader::new(fragment_kind.example(), fragment_kind)
    ///     .unwrap();
    /// let program = program::ShaderProgram::new(&vertex_shader, &fragment_shader).unwrap();
    /// assert!(program_questions::is_program(program.as_gl_id()).is_ok());
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
            program_questions::is_linked(program)?;
            program
        };

        Ok(ShaderProgram(program))
    }
}

impl gl::GlObject for ShaderProgram {
    fn as_gl_id(&self) -> GLuint {
        self.0
    }
}

#[derive(Debug, Clone)]
pub enum ProgramError {
    CompilationError(String),
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
