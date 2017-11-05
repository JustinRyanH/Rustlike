use std::{ffi, ptr};

use error::AppResult;
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
pub struct CompiledShader {
    /// Reference to Shader Allocation in the GPU
    glid: GLuint,
    /// Kind of Shader created
    kind: ShaderKind,
}

impl CompiledShader {
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
                panic!("Didn't Compile");
            }

            Ok(CompiledShader { glid, kind })
        }


    }
}

pub fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::raw::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = ffi::CString::new(src.as_bytes()).unwrap();
        gl::raw::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::raw::CompileShader(shader);

        // Get the compile status
        let mut status = gl::raw::FALSE as GLint;
        gl::raw::GetShaderiv(shader, gl::raw::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::raw::TRUE as GLint) {
            let mut len = 0;
            gl::raw::GetShaderiv(shader, gl::raw::INFO_LOG_LENGTH, &mut len);
            let mut buf: Vec<u8> = Vec::with_capacity(len as usize);
            gl::raw::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );

            buf.set_len(len as usize);

            panic!(
                "{}",
                String::from_utf8(buf).ok().expect(
                    "ShaderInfoLog not valid utf8",
                )
            );
        }
    }
    shader
}
