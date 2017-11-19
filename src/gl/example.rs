use gl::{AttributeCollection, AttributeKind, IntoAttributeCollection};
use gl::vertex::{VertexAttributes, Attribute, AttributeSize};

#[derive(Clone, Debug)]
pub struct ExampleVertex {
    pub pos: [f32; 3],
}

impl VertexAttributes for ExampleVertex {
    #[inline]
    unsafe fn attributes() -> Vec<Attribute> {
        use std::ptr;
        vec![
            Attribute::new(
                AttributeSize::Three,
                AttributeKind::Float,
                false,
                &(*(ptr::null() as *const ExampleVertex)).pos as *const _ as usize,
            ),
        ]
    }
}

impl IntoAttributeCollection<ExampleVertex> for Vec<ExampleVertex> {}
impl Into<AttributeCollection<ExampleVertex>> for Vec<ExampleVertex> {
    fn into(self) -> AttributeCollection<ExampleVertex> {
        AttributeCollection {
            collection: self
        }
    }
}
