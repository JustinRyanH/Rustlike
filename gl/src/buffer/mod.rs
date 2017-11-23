mod array_object;
mod buffer_object;

use std::mem;
use std::os::raw::c_void;

pub use self::array_object::*;
pub use self::buffer_object::*;

use GlObject;
use raw;
use errors::GlResult;
use attributes;

pub struct BufferConfiguration<A>
where
    A: attributes::DescribeAttributes,
{
    vertices: Vec<A>,
}

impl<A> BufferConfiguration<A>
where
    A: attributes::DescribeAttributes,
{
    pub fn new(vertices: Vec<A>) -> BufferConfiguration<A> {
        BufferConfiguration { vertices: vertices.into() }
    }

    pub fn build(self) -> GlResult<BufferObject> {
        let mut vbo = GlBuffer::new(BufferKind::Array);
        let mut vao = VertexArrayObject::new();

        unsafe {
            let bounded_vao = vao.bind();
            // TODO: There needs to be a global GlContext that prevents this guy
            // from binding if someone of the same type is bounded. Otherwise build
            // errors.
            let bounded_vbo = vbo.bind(Some(&bounded_vao));
            bind_to_buffer(&self.vertices, &bounded_vbo)?;
            describe_to_buffer(&self.vertices, &bounded_vbo);
        }

        Ok(BufferObject {
            vao,
            vbo,
            ebo: None,
        })
    }
}

#[derive(Debug)]
pub struct BufferObject {
    vao: VertexArrayObject,
    vbo: GlBuffer,
    ebo: Option<GlBuffer>,
}


impl BufferObject {
    pub fn draw(&self) -> GlResult<()> {
        unsafe {
            raw::BindVertexArray(self.vao.as_gl_id());
            raw::DrawArrays(raw::TRIANGLES, 0, 3)
        }
        Ok(())
    }
}

#[inline]
pub unsafe fn bind_to_buffer<A>(vertices: &Vec<A>, bounded_buffer: &BoundGlBuffer) -> GlResult<()>
where
    A: attributes::DescribeAttributes,
{
    let size = (vertices.len() * mem::size_of::<A>()) as isize;
    raw::BufferData(
        bounded_buffer.kind().into(),
        size,
        &vertices[0] as *const A as *const c_void,
        raw::STATIC_DRAW,
    );
    Ok(())
}

#[inline]
pub unsafe fn describe_to_buffer<A>(_: &Vec<A>, bounded_buffer: &BoundGlBuffer)
where
    A: attributes::DescribeAttributes,
{
    for (index, attribute) in A::attributes().iter().enumerate() {
        attribute.describe_to_gl(bounded_buffer, index as u32)
    }
}
