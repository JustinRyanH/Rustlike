extern crate rl_gl;
#[macro_use]
extern crate rl_gl_derive;
extern crate rspec;

#[cfg(test)]
mod tests {
    use super::*;
    use rl_gl::attributes::{Attribute, AttributeSize, DescribeAttributes};
    use rspec::given;


    #[test]
    fn deriving_describe_attributes() {
        let tested: Vec<Attribute> = Vec::new();
        rspec::run(&given("DescribeAttributes", tested, |ctx| {
            ctx.when("struct contains attributes with floats", |ctx| {
                ctx.before_each(|example| {
                    #[derive(DescribeAttributes)]
                    struct ExampleStruct {
                        color: [f32; 4],
                        position: [f32; 3],
                        uv: [f32; 2],
                        // another_var: f32,
                    }
                    unsafe {
                        example.extend(ExampleStruct::attributes().iter().cloned());
                    }
                });
                ctx.then("it has four attributes", |example| {
                    assert_eq!(example.len(), 4)
                });
                ctx.then("each attribute is the right size", |example| {
                    let size_slice: Vec<AttributeSize> = example.iter().map(|a| a.size()).collect();
                    assert_eq!(
                        size_slice,
                        vec![
                            AttributeSize::Four,
                            AttributeSize::Three,
                            AttributeSize::Two,
                            // AttributeSize::One,
                        ]
                    );
                })
            });
        }));
    }
}
