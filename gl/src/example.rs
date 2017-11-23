use std::mem;

use DescribeAttributes;
use attributes::{Attribute, AttributeSize, AttributeKind};

#[derive(Clone, Debug)]
pub struct ExampleVertex {
    pub pos: [f32; 3],
    pub set: [i32; 2]
}

impl DescribeAttributes for ExampleVertex {
    #[inline]
    unsafe fn attributes() -> Vec<Attribute> {
        use std::ptr;
        vec![
            Attribute::new(
                AttributeSize::Three,
                AttributeKind::Float,
                false,
                mem::size_of::<ExampleVertex>(),
                &(*(ptr::null() as *const ExampleVertex)).pos as *const _ as usize
            ),
            Attribute::new(
                AttributeSize::Two,
                AttributeKind::Int,
                false,
                mem::size_of::<ExampleVertex>(),
                &(*(ptr::null() as *const ExampleVertex)).set as *const _ as usize
            ),
        ]
    }
}

#[cfg(test)]
extern crate glutin;

#[cfg(test)]
pub fn headless_gl_window() -> ((), glutin::HeadlessContext) {
    use example::glutin::GlContext;
    use raw;

    let width: i32 = 256;
    let height: i32 = 256;
    let window = glutin::HeadlessRendererBuilder::new(width as u32, height as u32)
        .build()
        .unwrap();

    unsafe { window.make_current().expect("Couldn't make window current") };
    let gl = raw::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    return (gl, window);
}

#[cfg(not(test))]
pub fn headless_gl_window() -> ((), ()) {
    unimplemented!();
}
