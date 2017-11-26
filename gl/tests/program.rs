extern crate glutin;
extern crate rl_gl;

use glutin::GlContext;
use rl_gl::raw::types::*;

#[cfg(test)]
fn headless_gl_window() -> ((), glutin::HeadlessContext) {
    let width: i32 = 256;
    let height: i32 = 256;
    let window = glutin::HeadlessRendererBuilder::new(width as u32, height as u32)
        .build()
        .unwrap();

    unsafe { window.make_current().expect("Couldn't make window current") };
    let gl = rl_gl::raw::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    return (gl, window);
}

#[cfg(test)]
mod spec {
    use rl_gl::GlObject;
    use super::*;

    #[test]
    fn shaders_compile() {
        static VS_SRC: &'static str = "#version 150\n\
                                       in vec2 position;\n\
                                       void main() {\n\
                                       gl_Position = vec4(position, 0.0, 1.0);\n\
                                       }";

        static FS_SRC: &'static str = "#version 150\n\
                                       out vec4 out_color;\n\
                                       void main() {\n\
                                       out_color = vec4(1.0, 1.0, 1.0, 1.0);\n\
                                       }";

        let (_gl, _window) = headless_gl_window();
        let vs_id: GLuint;
        let _program = {
            let vs =
                rl_gl::program::CompiledShader::new(VS_SRC, rl_gl::program::ShaderKind::Vertex)
                    .unwrap();
            let fs =
                rl_gl::program::CompiledShader::new(FS_SRC, rl_gl::program::ShaderKind::Fragment)
                    .unwrap();
            vs_id = vs.as_gl_id();
            rl_gl::program::ShaderProgram::new(&vs, &fs).unwrap()
        };

        assert!(rl_gl::program::questions::shader::is_shader(vs_id).is_ok());
        assert_eq!(
            rl_gl::program::questions::shader::is_deleted(vs_id).unwrap(),
            true
        );
    }

    #[test]
    fn shaders_fail_if_shader_is_bad() {
        static VS_SRC: &'static str = "#version 150\n\
                                       in vec2 position;\n\
                                       void main() {\n\
                                       gl_Position = vec4(position, 1.0);\n\
                                       }";
        let (_gl, _window) = headless_gl_window();
        assert!(
            rl_gl::program::CompiledShader::new(VS_SRC, rl_gl::program::ShaderKind::Vertex).is_err()
        );
    }
}
