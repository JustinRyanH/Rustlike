use DescribeAttributes;
use attributes::{Attribute, AttributeSize, AttributeKind, IntoAttributeCollection,
                 AttributeCollection};

#[derive(Clone, Debug)]
pub struct ExampleVertex {
    pub pos: [f32; 3],
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
                &(*(ptr::null() as *const ExampleVertex)).pos as *const _ as usize
            ),
        ]
    }
}

impl IntoAttributeCollection<ExampleVertex> for Vec<ExampleVertex> {}
impl Into<AttributeCollection<ExampleVertex>> for Vec<ExampleVertex> {
    fn into(self) -> AttributeCollection<ExampleVertex> {
        AttributeCollection::new(self)
    }
}
