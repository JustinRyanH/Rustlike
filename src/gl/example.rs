use std::mem;
use std::marker::PhantomData;

use gl::{AttributeCollection, AttributeKind};
use gl::vertex::{VertexAttributes, Attribute, AttributeSize};
use gl::raw::types::*;

#[derive(Clone, Debug)]
pub struct ExampleVertex {
    pub pos: [f32; 3],
}

impl VertexAttributes for ExampleVertex {
    #[inline]
    fn attributes() -> Vec<Attribute> {
        [
            Attribute::new(
                AttributeSize::Three,
                AttributeKind::Float,
                false,
                3 * mem::size_of::<GLfloat>() as usize,
            ),
        ].to_vec()
    }
}


impl Into<Vec<f32>> for ExampleVertex {
    fn into(self) -> Vec<f32> {
        [self.pos[0], self.pos[1], self.pos[2]].to_vec()
    }
}

impl Into<AttributeCollection<f32, ExampleVertex>> for Vec<ExampleVertex> {
    fn into(self) -> AttributeCollection<f32, ExampleVertex> {
        let collection = self.iter()
            .flat_map::<Vec<f32>, _>(|v| v.clone().into())
            .collect();
        AttributeCollection {
            collection,
            attributes: PhantomData,
        }
    }
}
