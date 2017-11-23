extern crate rl_gl;
#[macro_use]
extern crate rl_gl_derive;
extern crate rspec;

#[cfg(test)]
mod tests {
    use super::*;
    use rl_gl::DescribeAttributes;
    use rl_gl::attributes::{Attribute, AttributeSize, AttributeKind};
    use rspec::given;


    #[test]
    fn deriving_describe_attributes() {
        let tested: Vec<Attribute> = Vec::new();
        rspec::run(&given("DescribeAttributes", tested, |ctx| {
            ctx.when("struct contains attributes with floats", |ctx| {
                ctx.before_each(|example| {
                    #[derive(Clone, DescribeAttributes)]
                    struct ExampleStruct {
                        _color: [f32; 4],
                        _position: [f32; 3],
                        _uv: [f32; 2],
                        _another_var: f32,
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
                            AttributeSize::One,
                        ]
                    );
                })
            });
            ctx.when("struct contains attributes with integers", |ctx| {
                ctx.before_each(|example| {
                    #[derive(Clone, DescribeAttributes)]
                    struct ExampleStruct {
                        _uv: [i32; 2],
                        _something: i32,
                    }
                    unsafe {
                        example.extend(ExampleStruct::attributes().iter().cloned());
                    }
                });
                ctx.then("each attribute is the right size", |example| {
                    let size_slice: Vec<AttributeSize> = example.iter().map(|a| a.size()).collect();
                    assert_eq!(size_slice, vec![AttributeSize::Two, AttributeSize::One]);
                });
                ctx.then("all attributes are integer", |example| {
                    let kind_slice: Vec<AttributeKind> = example.iter().map(|a| a.kind()).collect();
                    assert_eq!(kind_slice, vec![AttributeKind::Int, AttributeKind::Int]);
                });
            });
            ctx.when("struct are a mix of primitives", |ctx| {
                ctx.before_each(|example| {
                    #[derive(Clone, DescribeAttributes)]
                    struct ExampleStruct {
                        _float: f32,
                        _u_int: u32,
                        _signed_int: i32,
                        _doube: f64,
                        _byte: i8,
                        _u_byte: u8,
                        _short: i16,
                        _u_short: u16,
                    }
                    unsafe {
                        example.extend(ExampleStruct::attributes().iter().cloned());
                    }
                });
                ctx.then("describe the attributes in the right order", |example| {
                    let kind_slice: Vec<AttributeKind> = example.iter().map(|a| a.kind()).collect();
                    assert_eq!(
                        kind_slice,
                        vec![
                            AttributeKind::Float,
                            AttributeKind::UnsignedInt,
                            AttributeKind::Int,
                            AttributeKind::Double,
                            AttributeKind::Byte,
                            AttributeKind::UnsignedByte,
                            AttributeKind::Short,
                            AttributeKind::UnsignedShort,
                        ]
                    );
                });
            });
        }));
    }
}
