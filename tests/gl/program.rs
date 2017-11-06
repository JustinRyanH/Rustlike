use rustlike::context::ContextBuilder;
use rustlike::gl::program::{self, ShaderKind};


#[cfg(test)]
mod shader_tests {
    use super::*;

    #[test]
    fn handles_bad_shaders() {
        const BAD_VERTEX: &'static str =  r"
            #version 330 core
            layout (location = 0) in vec3 aPos;
            void main()
            {
                gl_Position = vec4(aPos.y, aPos.z, 1.0);
            }";
        let _ctx = ContextBuilder::default().build().unwrap();
        let vertex_kind = ShaderKind::Vertex;
        let vertex_shader = program::CompiledShader::new(BAD_VERTEX, vertex_kind);
        vertex_shader.expect_err("Too few arguments to constructor of 'vec4'");
    }
}
