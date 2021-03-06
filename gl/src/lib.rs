#![warn(missing_docs)]
//! is OpenGL Wrapper that attempts to be Type Safe
//! as well as preferment

#[cfg(test)]
extern crate rspec;

#[allow(missing_docs)]
pub mod raw;
pub mod errors;
pub mod attributes;
pub mod program;
pub mod buffer;

pub use self::attributes::{Attribute, AttributeKind};
pub use self::program::uniforms::Uniform;

/// Trait used to Describe Rust struct fields to OpenGL buffers
pub use self::attributes::DescribeAttributes;
pub use self::program::uniforms::UpdatableUniforms;

/// All OpenGL objects have an id which uses to
/// tell the driver to perform commands on them.
/// However, sometimes questions needed to be asked
/// about an object after original abstraction has
/// been Dropped
pub trait GlObject {
    /// Gets the Id of the GlObejct
    fn as_gl_id(&self) -> raw::types::GLuint;
}
