extern crate rl_gl;
extern crate rspec;

use rl_gl::uniforms::{Uniform};
use self::rspec::given;

#[test]
fn main() {
    // rspec::run(&given("an UniformSize", UniformSize::default(), |ctx| {
    //     ctx.when("an Uniform Size represents a `2`", |ctx| {
    //         ctx.before(|example| *example = UniformSize::Two);
    //         ctx.then("it can be cast into an `i32`", |example| {
    //             assert_eq!(2 as i32, example.clone().into());
    //         });
    //         ctx.then("it can be cast into an `u32`", |example| {
    //             assert_eq!(2 as u32, example.clone().into());
    //         });
    //         ctx.then("it can be cast into an `usize`", |example| {
    //             assert_eq!(2 as usize, example.clone().into());
    //         });
    //     });

    //     ctx.when("an Uniform Size represents a `3`", |ctx| {
    //         ctx.before(|example| *example = UniformSize::Three);
    //         ctx.then("it can be cast into an `i32`", |example| {
    //             assert_eq!(3 as i32, example.clone().into());
    //         });
    //         ctx.then("it can be cast into an `u32`", |example| {
    //             assert_eq!(3 as u32, example.clone().into());
    //         });
    //         ctx.then("it can be cast into an `usize`", |example| {
    //             assert_eq!(3 as usize, example.clone().into());
    //         });
    //     });

    //     ctx.when("an Uniform Size represents a `4`", |ctx| {
    //         ctx.before(|example| *example = UniformSize::Four);
    //         ctx.then("it can be cast into an `i32`", |example| {
    //             assert_eq!(4 as i32, example.clone().into());
    //         });
    //         ctx.then("it can be cast into an `u32`", |example| {
    //             assert_eq!(4 as u32, example.clone().into());
    //         });
    //         ctx.then("it can be cast into an `usize`", |example| {
    //             assert_eq!(4 as usize, example.clone().into());
    //         });
    //     });
    // }));

    // rspec::run(&given("an Uniform", Uniform::default(), |ctx| {
    //     ctx.when("casting an `i32` into Uniform", |ctx| {
    //         ctx.before_each(|example| {
    //             let target: i32 = 0;
    //             *example = target.into()
    //         });

    //         ctx.then("it turns into a Uniform::Scalar as a Int", |example| {
    //             assert_eq!(&Uniform::Scalar(UniformKind::Int), example);
    //         });
    //     });
    //     ctx.when("casting into an Scalar Unsigned Int Uniform", |ctx| {
    //         ctx.before_each(|example| {
    //             *example = Uniform::Scalar(UniformKind::UnsignedInt)
    //         });

    //         ctx.then("it made from an u32", |example| {
    //             let target: u32 = 0;
    //             assert_eq!(example.clone(), target.into());
    //         });

    //         ctx.then("it made from an u16", |example| {
    //             let target: u16 = 0;
    //             assert_eq!(example.clone(), target.into());
    //         });

    //         ctx.then("it made from an u8", |example| {
    //             let target: u8 = 0;
    //             assert_eq!(example.clone(), target.into());
    //         });
    //     });

    //     ctx.when("casting into an Scalar Int Uniform", |ctx| {
    //         ctx.before_each(|example| {
    //             *example = Uniform::Scalar(UniformKind::Int)
    //         });

    //         ctx.then("it made from an i32", |example| {
    //             let target: i32 = 0;
    //             assert_eq!(example.clone(), target.into());
    //         });

    //         ctx.then("it made from an i16", |example| {
    //             let target: i16 = 0;
    //             assert_eq!(example.clone(), target.into());
    //         });

    //         ctx.then("it made from an i8", |example| {
    //             let target: i8 = 0;
    //             assert_eq!(example.clone(), target.into());
    //         });
    //     });

    //     ctx.when("casting into an Scalar Float Uniform", |ctx| {
    //         ctx.before_each(|example| {
    //             *example = Uniform::Scalar(UniformKind::Float)
    //         });

    //         ctx.then("it made from an f32", |example| {
    //             let target: f32 = 0.;
    //             assert_eq!(example.clone(), target.into());
    //         });
    //     });

    //     ctx.when("casting into an Scalar Bool Uniform", |ctx| {
    //         ctx.before_each(|example| {
    //             *example = Uniform::Scalar(UniformKind::Bool)
    //         });

    //         ctx.then("it made from an f32", |example| {
    //             let target: bool = false;
    //             assert_eq!(example.clone(), target.into());
    //         });
    //     });
    // }));
}
