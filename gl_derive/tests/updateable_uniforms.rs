/// TODO: Custom gl_generator that acts as a mock.
extern crate rl_gl;
#[macro_use]
extern crate rl_gl_derive;
extern crate rspec;

#[cfg(test)]
mod tests {
    use super::*;
    use rl_gl::UpdatableUniforms;
    use rspec::given;


    #[test]
    fn deriving_describe_attributes() {
        let tested: Vec<Attribute> = Vec::new();
        rspec::run(&given("UpdatableUniforms", tested, |ctx| {}));
    }
}
