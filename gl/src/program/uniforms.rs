//! Enums to represent Uniforms for Shader Programs


/// Implement this if you want to update the
pub trait UpdatableUniforms {
    /// Returns a list of named uniform values.
    fn uniform_values(&self) -> Vec<NamedUniform>;
    /// Returns a list changed values that need updates
    fn changed_uniform_values(&self) -> Vec<NamedUniform>;
}


/// Structure used to send data to OpenGL
pub struct NamedUniform {
    uniform: Uniform,
    /// Name should always be static because it
    /// needs to be known at compile-time
    name: &'static str,
}

/// Uniforms that are 1byN column matrices
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

/// Representation of GlSl Uniform Values
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
