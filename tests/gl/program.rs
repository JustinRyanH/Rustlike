use rustlike::context::ContextBuilder;
use rustlike::gl::program::{self, ShaderKind};

#[cfg(test)]
mod shader_tests {
    use super::*;

    #[test]
    fn test() {
        let _ctx = ContextBuilder::default().build().unwrap();
        // TODO This could totally be moved into rspec
        /// When Shader is given bad Source, it returns an error
        {
            const BAD_VERTEX: &'static str = r"
            #version 330 core
            layout (location = 0) in vec3 aPos;
            void main()
            {
                gl_Position = vec4(aPos.y, aPos.z, 1.0);
            }";
            let vertex_kind = ShaderKind::Vertex;
            let vertex_shader = program::CompiledShader::new(BAD_VERTEX, vertex_kind);
            vertex_shader.expect_err("Too few arguments to constructor of 'vec4'");
        }

        /// When the Shader is Dropped, it cleans up the shader
        {
            let gl_id = {
                let vertex_kind = ShaderKind::Vertex;
                let vertex_shader = program::CompiledShader::new(vertex_kind.example(), vertex_kind)
                    .unwrap();
                assert_eq!(program::shader_questions::shader_kind(vertex_shader.as_gl_id())
                           .unwrap(),
                           vertex_kind);
                assert!(program::shader_questions::is_shader(vertex_shader.as_gl_id()).is_ok());
                vertex_shader.as_gl_id()

            };
            program::shader_questions::is_shader(gl_id).expect_err(
                format!("{} is not referencing a glShader", gl_id).as_str()
            );
        }
    }
}
