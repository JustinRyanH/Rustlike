//! Enums to represent Uniforms for Shader Programs
use std::ffi;
use GlObject;
use raw;
use program::ShaderProgram;
use errors::GlResult;
// UniformBlockBinding
// UniformMatrix2fv
// UniformMatrix2x3fv
// UniformMatrix2x4fv
// UniformMatrix3fv
// UniformMatrix3x2fv
// UniformMatrix3x4fv
// UniformMatrix4fv
// UniformMatrix4x2fv
// UniformMatrix4x3f

pub struct UniformScalar<T>(T);

impl From<i32> for UniformScalar<i32> {
    fn from(v: i32) -> UniformScalar<i32> {
        UniformScalar(v)
    }
}

impl From<u32> for UniformScalar<u32> {
    fn from(v: u32) -> UniformScalar<u32> {
        UniformScalar(v)
    }
}

impl From<f32> for UniformScalar<f32> {
    fn from(v: f32) -> UniformScalar<f32> {
        UniformScalar(v)
    }
}

pub enum UniformVector<T>
where
    T: Into<UniformScalar<T>>,
{
    TwoDimensions(T, T),
    ThreeDimensions(T, T, T),
    FourDimensions(T, T, T, T),
}

impl<T> From<[T; 2]> for UniformVector<T>
where
    T: Into<UniformScalar<T>> + Clone + Copy,
{
    fn from(v: [T; 2]) -> UniformVector<T> {
        UniformVector::TwoDimensions(v[0], v[1])
    }
}

impl<T> From<[T; 3]> for UniformVector<T>
where
    T: Into<UniformScalar<T>> + Clone + Copy,
{
    fn from(v: [T; 3]) -> UniformVector<T> {
        UniformVector::ThreeDimensions(v[0], v[1], v[2])
    }
}

impl<T> From<[T; 4]> for UniformVector<T>
where
    T: Into<UniformScalar<T>> + Clone + Copy,
{
    fn from(v: [T; 4]) -> UniformVector<T> {
        UniformVector::FourDimensions(v[0], v[1], v[2], v[3])
    }
}

/// Representation of GlSl Uniform Values
pub enum Uniform {
    ScalarFloat(UniformScalar<f32>),
    ScalarInt(UniformScalar<i32>),
    ScalarUnsignedInt(UniformScalar<u32>),
    VectorFloat(UniformVector<f32>),
    VectorInt(UniformVector<i32>),
    VectorUnsignedInt(UniformVector<u32>),
}

impl From<i32> for Uniform {
    fn from(v: i32) -> Uniform {
        Uniform::ScalarInt(v.into())
    }
}

impl From<u32> for Uniform {
    fn from(v: u32) -> Uniform {
        Uniform::ScalarUnsignedInt(v.into())
    }
}

impl From<f32> for Uniform {
    fn from(v: f32) -> Uniform {
        Uniform::ScalarFloat(v.into())
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
