extern crate rustlike;
extern crate rspec;
extern crate expect;


use rustlike::gl::AttributeKind;
use rspec::given;

#[test]
fn main() {
    rspec::run(&given("an AttributeKind", AttributeKind::Byte, |ctx| {
        ctx.when("the type is a GlByte", |ctx| {
            ctx.it("then has size of 1 byte", |example| {
                assert_eq!(example.size_of(), 1)
            });
        });
        ctx.when("the type is an Unsigned GlByte", |ctx| {
            ctx.before_each(|example| *example = AttributeKind::UnsignedByte);
            ctx.it("then has size of 1 byte", |example| {
                assert_eq!(example.size_of(), 1)
            });
        });
        ctx.when("the type is an GlShort", |ctx| {
            ctx.before_each(|example| *example = AttributeKind::Short);
            ctx.it("then has size of 2 bytes", |example| {
                assert_eq!(example.size_of(), 2)
            });
        });
        ctx.when("the type is an Unsigned GlShort", |ctx| {
            ctx.before_each(|example| *example = AttributeKind::UnsignedShort);
            ctx.it("then has size of 2 bytes", |example| {
                assert_eq!(example.size_of(), 2)
            });
        });
        ctx.when("the type is a Glint", |ctx| {
            ctx.before_each(|example| *example = AttributeKind::Int);
            ctx.it("then has size of 4 bytes", |example| {
                assert_eq!(example.size_of(), 4)
            });
        });
        ctx.when("the type is a Unsigned Glint", |ctx| {
            ctx.before_each(|example| *example = AttributeKind::UnsignedInt);
            ctx.it("then has size of 4 bytes", |example| {
                assert_eq!(example.size_of(), 4)
            });
        });
        ctx.when("the type is a GLfloat", |ctx| {
            ctx.before_each(|example| *example = AttributeKind::Float);
            ctx.it("then has size of 4 bytes", |example| {
                assert_eq!(example.size_of(), 4)
            });
        });
        ctx.when("the type is a Gldouble", |ctx| {
            ctx.before_each(|example| *example = AttributeKind::Double);
            ctx.it("then has size of 8 bytes", |example| {
                assert_eq!(example.size_of(), 8)
            });
        });
    }));
}
