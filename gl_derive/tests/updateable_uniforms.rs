/// TODO: Custom gl_generator that acts as a mock.
extern crate rl_gl;
#[macro_use]
extern crate rl_gl_derive;
extern crate rspec;

#[cfg(test)]
mod tests {
    use super::*;

    use rl_gl::UpdatableUniforms;
    use rl_gl::program::uniforms::{NamedUniform};
    use rspec::given;

    #[test]
    fn deriving_describe_attributes() {
        let tested: Vec<NamedUniform> = Vec::new();
        rspec::run(&given("UpdatableUniforms", tested, |ctx| {
            ctx.when("Getting all Uniform Values", |ctx| {
                ctx.before_each(|example| {
                    #[derive(Clone, UpdatableUniforms)]
                    struct ExampleUniformCollection {
                        a_float: f32,
                        vec_float: [f32; 4],
                    }
                    let instance = ExampleUniformCollection {
                        a_float: 0.,
                        vec_float: [0., 1., 3., 4.],
                    };
                    *example = instance.uniform_values();
                });

                ctx.then("then it returns ", |example| {
                    assert_eq!(
                        *example,
                        vec![
                            NamedUniform::new("a_float", 0.),
                            NamedUniform::new("vec_float", [0., 1., 3., 4.]),
                        ]
                    )
                })

            });
            ctx.when("Getting Changed Uniform Values", |ctx| {
                ctx.before_each(|example| {
                    #[derive(Clone, UpdatableUniforms)]
                    struct ExampleUniformCollection {
                        a_float: f32,
                        vec_float: [f32; 4],
                        to_update: Vec<&'static str>,
                    }
                    let mut to_update: Vec<&'static str> = Vec::new();
                    to_update.push("a_float");
                    let mut instance = ExampleUniformCollection {
                        a_float: 0.,
                        vec_float: [0., 1., 3., 4.],
                        to_update,
                    };
                    *example = instance.changed_uniform_values();
                });

                ctx.then("then it returns ", |example| {
                    assert_eq!(*example, vec![NamedUniform::new("a_float", 0.)])
                })
            });
        }));
    }
}
