//! Enums to represent Uniforms for Shader Programs

/// Used to communicate new attributes to an OpenGL Drive
///
/// UpdatableUniforms comes with a derive attribute that
/// will return all of the attributes on a struct every time
/// for both `changed_uniform_values` and `uniform_values`
/// however if you implement it with an `to_update` field
/// that takes some `Iterator` of static strings it will only return
/// the fields that are in the iterator and clear the iterator.
///
/// ```
/// # #[macro_use] extern crate rl_gl_derive;
/// # extern crate rl_gl;
/// #
/// # fn main() {
/// use rl_gl::UpdatableUniforms;
/// use rl_gl::program::uniforms::NamedUniform;
///
/// #[derive(Clone, Debug, UpdatableUniforms)]
/// struct Color {
///     color: [f32; 4],
///     x_offset: f32,
///     to_update: Vec<&'static str>,
/// };
///
/// let mut example_color = Color{
///   color: [1., 0., 1., 1.],
///   x_offset: 0.,
///   to_update: vec![ "color" ],
/// };
/// let needs_update = example_color.changed_uniform_values();
/// assert_eq!(needs_update.first().unwrap(), &NamedUniform::new("color", [1., 0., 1., 1.]))
/// # }
/// ```
pub trait UpdatableUniforms {
    /// Returns a list of named uniform values.
    fn uniform_values(&self) -> Vec<NamedUniform>;
    /// Returns a list changed values that need updates
    fn changed_uniform_values(&mut self) -> Vec<NamedUniform>;
}


/// Structure used to send data to OpenGL
#[derive(Debug, Clone, PartialEq)]
pub struct NamedUniform {
    uniform: Uniform,
    /// Name should always be static because it
    /// needs to be known at compile-time
    name: &'static str,
}

impl NamedUniform {
    /// Creates a new Named Uniform
    pub fn new<T>(name: &'static str, uniform: T) -> NamedUniform
    where
        T: Into<Uniform>,
    {
        NamedUniform {
            name,
            uniform: uniform.into(),
        }
    }
    /// Gets the name of the Named Uniform
    #[inline]
    pub fn name(&self) -> &'static str {
        self.name
    }
    /// Gets the Uniform value of the Named Uniform
    pub fn value(self) -> Uniform {
        self.uniform
    }
}

/// Uniforms that are 1byN column matrices
#[derive(Debug, Clone, PartialEq)]
pub enum UniformVector<T>
where
    T: Into<Uniform>,
{
    /// `[T; 2]`
    TwoDimensions(T, T),
    /// `[T; 3]`
    ThreeDimensions(T, T, T),
    /// `[T; 4]`
    FourDimensions(T, T, T, T),
}

impl<T> From<[T; 2]> for UniformVector<T>
where
    T: Into<Uniform> + Clone + Copy,
{
    fn from(v: [T; 2]) -> UniformVector<T> {
        UniformVector::TwoDimensions(v[0], v[1])
    }
}

impl<T> From<[T; 3]> for UniformVector<T>
where
    T: Into<Uniform> + Clone + Copy,
{
    fn from(v: [T; 3]) -> UniformVector<T> {
        UniformVector::ThreeDimensions(v[0], v[1], v[2])
    }
}

impl<T> From<[T; 4]> for UniformVector<T>
where
    T: Into<Uniform> + Clone + Copy,
{
    fn from(v: [T; 4]) -> UniformVector<T> {
        UniformVector::FourDimensions(v[0], v[1], v[2], v[3])
    }
}

/// Uniforms that are NxN matrices
#[derive(Debug, Clone, PartialEq)]
pub enum UniformMatrix {
    /// GLSL `mat4`
    Mat4([[f32; 4]; 4]),
    /// GLSL `mat3`
    Mat3([[f32; 3]; 3]),
    /// GLSL `mat2`
    Mat2([[f32; 2]; 2]),
}

/// Representation of GlSl Uniform Values
#[derive(Debug, Clone, PartialEq)]
pub enum Uniform {
    /// GLSL `float`
    ScalarFloat(f32),
    /// GLSL `int`
    ScalarInt(i32),
    /// GLSL `uint`
    ScalarUnsignedInt(u32),
    /// GLSL `vecn`
    VectorFloat(UniformVector<f32>),
    /// GLSL `ivecn`
    VectorInt(UniformVector<i32>),
    /// GLSL `uvecn`
    VectorUnsignedInt(UniformVector<u32>),
    /// GLSL `matn`
    Matrix(UniformMatrix),
}

impl From<i32> for Uniform {
    fn from(v: i32) -> Uniform {
        Uniform::ScalarInt(v)
    }
}

impl From<u32> for Uniform {
    fn from(v: u32) -> Uniform {
        Uniform::ScalarUnsignedInt(v)
    }
}

impl From<f32> for Uniform {
    fn from(v: f32) -> Uniform {
        Uniform::ScalarFloat(v)
    }
}

impl From<[f32; 2]> for Uniform {
    fn from(v: [f32; 2]) -> Uniform {
        Uniform::VectorFloat(v.into())
    }
}

impl From<[f32; 3]> for Uniform {
    fn from(v: [f32; 3]) -> Uniform {
        Uniform::VectorFloat(v.into())
    }
}

impl From<[f32; 4]> for Uniform {
    fn from(v: [f32; 4]) -> Uniform {
        Uniform::VectorFloat(v.into())
    }
}

impl From<[u32; 2]> for Uniform {
    fn from(v: [u32; 2]) -> Uniform {
        Uniform::VectorUnsignedInt(v.into())
    }
}

impl From<[u32; 3]> for Uniform {
    fn from(v: [u32; 3]) -> Uniform {
        Uniform::VectorUnsignedInt(v.into())
    }
}

impl From<[u32; 4]> for Uniform {
    fn from(v: [u32; 4]) -> Uniform {
        Uniform::VectorUnsignedInt(v.into())
    }
}

impl From<[i32; 2]> for Uniform {
    fn from(v: [i32; 2]) -> Uniform {
        Uniform::VectorInt(v.into())
    }
}

impl From<[i32; 3]> for Uniform {
    fn from(v: [i32; 3]) -> Uniform {
        Uniform::VectorInt(v.into())
    }
}

impl From<[i32; 4]> for Uniform {
    fn from(v: [i32; 4]) -> Uniform {
        Uniform::VectorInt(v.into())
    }
}

impl From<[[f32; 2]; 2]> for Uniform {
    fn from(v: [[f32; 2]; 2]) -> Uniform {
        Uniform::Matrix(UniformMatrix::Mat2(v))
    }
}


impl From<[[f32; 3]; 3]> for Uniform {
    fn from(v: [[f32; 3]; 3]) -> Uniform {
        Uniform::Matrix(UniformMatrix::Mat3(v))
    }
}

impl From<[[f32; 4]; 4]> for Uniform {
    fn from(v: [[f32; 4]; 4]) -> Uniform {
        Uniform::Matrix(UniformMatrix::Mat4(v))
    }
}
