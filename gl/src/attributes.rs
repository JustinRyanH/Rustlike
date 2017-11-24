//! Tools to communicate
//! how Rust structs and types can be used by
//! the OpenGL driver with Vertices

use std::{mem, ops};
use std::fmt::Debug;
use std::os::raw::c_void;

use raw;
use raw::types::*;
use buffer::BoundBufferObject;

/// creates a series of OpenGL friendly
/// attributes use to bind vertices to buffers
///
/// # Example Manual Implementation
/// ```
/// use rl_gl::DescribeAttributes;
/// use rl_gl::attributes::{Attribute, AttributeSize, AttributeKind};
///
/// #[derive(Clone, Debug)]
/// struct Vertices {
///     position: [f32; 3],
///     color: [u8; 4],
/// }
///
/// impl DescribeAttributes for Vertices {
///     unsafe fn attributes() -> Vec<Attribute> {
///         use std::mem;
///         vec![
///             Attribute::new(
///                 AttributeSize::Three,
///                 AttributeKind::Float,
///                 false,
///                 mem::size_of::<Vertices>(),
///                 &(*(std::ptr::null() as *const Vertices)).position as *const _ as usize
///             ),
///             Attribute::new(
///                 AttributeSize::Four,
///                 AttributeKind::UnsignedByte,
///                 false,
///                 mem::size_of::<Vertices>(),
///                 &(*(std::ptr::null() as *const Vertices)).color as *const _ as usize
///             ),
///         ]
///     }
/// }
/// ```
/// However, you do not need to implement DescribeAttributes for each struct you want to turn
/// into vertices.
/// # Example Derive
/// ```
/// # #[macro_use] extern crate rl_gl_derive;
/// # extern crate rl_gl;
/// #
/// # fn main() {
/// use std::mem;
/// use rl_gl::DescribeAttributes;
/// use rl_gl::attributes::{ Attribute, AttributeKind, AttributeSize };
///
/// #[derive(Clone, Debug, DescribeAttributes)]
/// struct Vertices {
///     position: [f32; 3],
///     color: [u8; 4],
/// }
///
/// unsafe {
///   let attributes = Vertices::attributes();
///
///   assert_eq!(attributes.len(), 2);
///   assert_eq!(
///       attributes[0],
///       Attribute::new(
///           AttributeSize::Three,
///           AttributeKind::Float,
///           false,
///           mem::size_of::<Vertices>(),
///           &(*(std::ptr::null() as *const Vertices)).position as *const _ as usize,
///       )
///   )
/// }
/// # }
/// ```
pub trait DescribeAttributes: Clone {
    /// gets an Vec of Attributes describing each field.
    /// the Vec should be ordered to the order of the fields
    /// when implemented
    unsafe fn attributes() -> Vec<Attribute>
    where
        Self: Sized;
}

/// describes size of the data if stored in an array. AttributeSize::One otherwise
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AttributeSize {
    /// represents `T` or `[T; 1]`
    One,
    /// represents `[T; 2]`
    Two,
    /// represents `[T; 3]`
    Three,
    /// represents `[T; 4]`
    Four,
}

/// Default::default() returns a 4 to match with OpenGL size API
/// # Example
/// ```
/// use rl_gl::attributes::AttributeSize;
///
/// assert_eq!(AttributeSize::Four, Default::default())
//  ```
impl Default for AttributeSize {
    fn default() -> AttributeSize {
        AttributeSize::Four
    }
}

impl Into<GLint> for AttributeSize {
    fn into(self) -> GLint {
        match self {
            AttributeSize::One => 1,
            AttributeSize::Two => 2,
            AttributeSize::Three => 3,
            AttributeSize::Four => 4,
        }
    }
}

/// Descriptions a Rust struct field or an array for OpenGL API consumption
#[derive(Clone, Debug, PartialEq)]
pub struct Attribute {
    size: AttributeSize,
    kind: AttributeKind,
    normalized: bool,
    stride: usize,
    point: usize,
}

impl Attribute {
    /// Attributes map to directly to [glVertexAttribPointer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribPointer.xhtml) calls
    pub fn new(
        size: AttributeSize,
        kind: AttributeKind,
        normalized: bool,
        stride: usize,
        point: usize,
    ) -> Attribute {
        Attribute {
            size,
            kind,
            normalized,
            stride,
            point,
        }
    }

    /// Gets the type of attribute
    pub fn kind(&self) -> AttributeKind {
        self.kind
    }

    /// Gets the size of the attribute
    pub fn size(&self) -> AttributeSize {
        self.size
    }

    /// Ask attribute if it is normalized
    pub fn normalized(&self) -> GLboolean {
        match self.normalized {
            true => raw::TRUE,
            false => raw::FALSE,
        }
    }

    /// Gets the byte offset between other attributes
    pub fn stride(&self) -> usize {
        self.stride
    }

    /// Gets the start point of the attribute when packed into an array
    pub fn point(&self) -> usize {
        self.point
    }

    /// Describes the Attribute to a bounded buffer
    pub unsafe fn describe_to_gl<'a>(&self, _: &BoundBufferObject<'a>, index: u32) {
        let size: GLint = self.size.into();
        raw::VertexAttribPointer(
            index,
            size,
            self.kind.into(),
            self.normalized(),
            self.stride as GLsizei,
            self.point as *const c_void,
        );
        raw::EnableVertexAttribArray(index);
    }
}

/// Represents Rust type for OpenGL API consumption
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AttributeKind {
    /// Attribute representation of `i8`
    Byte,
    /// Attribute representation of `u8`
    UnsignedByte,
    /// Attribute representation of `i16`
    Short,
    /// Attribute representation of `u16`
    UnsignedShort,
    /// Attribute representation of `i32`
    Int,
    /// Attribute representation of `u32`
    UnsignedInt,
    /// Attribute representation of `f32`
    Float,
    /// Attribute representation of `f64`
    Double,
}

impl AttributeKind {
    /// Returns the size of attribute in bytes as used by OpenGL
    /// #Example
    /// ```
    /// use rl_gl::AttributeKind;
    ///
    /// assert_eq!(1, AttributeKind::Byte.size_of());
    /// assert_eq!(8, AttributeKind::Double.size_of());
    ///
    /// ```
    pub fn size_of(self) -> usize {
        match self {
            AttributeKind::Byte => mem::size_of::<GLbyte>() as usize,
            AttributeKind::UnsignedByte => mem::size_of::<GLubyte>() as usize,
            AttributeKind::Short => mem::size_of::<GLshort>() as usize,
            AttributeKind::UnsignedShort => mem::size_of::<GLushort>() as usize,
            AttributeKind::Int => mem::size_of::<GLint>() as usize,
            AttributeKind::UnsignedInt => mem::size_of::<GLuint>() as usize,
            AttributeKind::Float => mem::size_of::<GLfloat>() as usize,
            AttributeKind::Double => mem::size_of::<GLdouble>() as usize,
        }
    }
}

/// Default::default() returns a Float to match with OpenGL API
/// # Example
/// ```
/// use rl_gl::attributes::AttributeKind;
///
/// assert_eq!(AttributeKind::Float, Default::default())
//  ```
impl Default for AttributeKind {
    fn default() -> AttributeKind {
        AttributeKind::Float
    }
}

impl From<f32> for AttributeKind {
    fn from(_: f32) -> AttributeKind {
        AttributeKind::Float
    }
}

impl From<i32> for AttributeKind {
    fn from(_: i32) -> AttributeKind {
        AttributeKind::Int
    }
}

impl Into<GLenum> for AttributeKind {
    fn into(self) -> GLenum {
        match self {
            AttributeKind::Byte => raw::BYTE,
            AttributeKind::UnsignedByte => raw::UNSIGNED_BYTE,
            AttributeKind::Float => raw::FLOAT,
            AttributeKind::Double => raw::DOUBLE,
            AttributeKind::Short => raw::SHORT,
            AttributeKind::UnsignedShort => raw::UNSIGNED_SHORT,
            AttributeKind::Int => raw::INT,
            AttributeKind::UnsignedInt => raw::UNSIGNED_INT,
        }
    }
}

/// A Type interface for sharing an AttributeKind and it's value
#[derive(Clone, PartialEq, Debug)]
pub struct AttributeValue<T>
where
    T: Into<AttributeKind> + PartialEq + Debug + Clone,
{
    value: T,
    kind: AttributeKind,
}

impl<T> AttributeValue<T>
where
    T: Into<AttributeKind> + PartialEq + Debug + Clone,
{
    /// Creates a AttributeValue, however `T.into()`
    /// the preferred method of creating an attribute value
    pub fn new(value: T) -> AttributeValue<T> {
        let kind = value.clone().into().clone();
        AttributeValue { value, kind }
    }

    /// Gets a copy of the Value
    pub fn value(&self) -> T {
        self.value.clone()
    }

    /// Gets a copy of the AttributeKind
    pub fn kind(&self) -> AttributeKind {
        self.kind.clone()
    }
}

/// # Example
/// ```
/// use rl_gl::attributes::{AttributeKind, AttributeValue};
///
/// let expected = AttributeValue::new(10.);
/// let value: f32 = 10.;
/// assert_eq!(expected, value.into())
/// ```
impl<T> From<T> for AttributeValue<T>
where
    T: Into<AttributeKind> + PartialEq + Debug + Clone,
{
    fn from(value: T) -> AttributeValue<T> {
        AttributeValue::new(value)
    }
}

impl<T> ops::Deref for AttributeValue<T>
where
    T: Into<AttributeKind> + PartialEq + Debug + Clone,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}
