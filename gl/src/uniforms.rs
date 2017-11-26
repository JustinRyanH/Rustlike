//! Enums to represent Uniforms for Shader Programs

// Uniform1f
// Uniform1fv
// Uniform1i
// Uniform1iv
// Uniform1ui
// Uniform1uiv
// Uniform2f
// Uniform2fv
// Uniform2i
// Uniform2iv
// Uniform2ui
// Uniform2uiv
// Uniform3f
// Uniform3fv
// Uniform3i
// Uniform3iv
// Uniform3ui
// Uniform3uiv
// Uniform4f
// Uniform4fv
// Uniform4i
// Uniform4iv
// Uniform4ui
// Uniform4uiv
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


/// Valid types Uniforms can be
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UniformKind {
    /// `i32`
    Int,
    /// `bool`
    Bool,
    /// `f32`
    Float,
    /// `u32`
    UnsignedInt,
}

impl From<i32> for UniformKind {
    fn from(_: i32) -> UniformKind {
        UniformKind::Int
    }
}

impl From<u32> for UniformKind {
    fn from(_: u32) -> UniformKind {
        UniformKind::UnsignedInt
    }
}

impl From<u16> for UniformKind {
    fn from(_: u16) -> UniformKind {
        UniformKind::UnsignedInt
    }
}

impl From<u8> for UniformKind {
    fn from(_: u8) -> UniformKind {
        UniformKind::UnsignedInt
    }
}

impl Default for UniformKind {
    fn default() -> UniformKind {
        UniformKind::Float
    }
}

/// Number of Columns or Rows of a Non-scalar uniform
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UniformSize {
    /// `[f32; 2]` for `[[f32; 2]; 2]`
    Two,
    /// `[f32; 3]` for `[[f32; 3]; 3]`
    Three,
    /// `[f32; 4]` for `[[f32; 4]; 4]`
    Four,
}

impl Default for UniformSize {
    fn default() -> UniformSize {
        UniformSize::Four
    }
}

impl UniformSize {
    fn as_usize(self) -> usize {
        match self {
            UniformSize::Two => 2,
            UniformSize::Three => 3,
            UniformSize::Four => 4,
        }
    }
}

impl Into<usize> for UniformSize {
    fn into(self) -> usize {
        self.as_usize()
    }
}

impl Into<i32> for UniformSize {
    fn into(self) -> i32 {
        self.as_usize() as i32
    }
}

impl Into<u32> for UniformSize {
    fn into(self) -> u32 {
        self.as_usize() as u32
    }
}


/// Rust representation of a GLSL Uniform
///
/// [more](https://www.khronos.org/opengl/wiki/Uniform_(GLSL)) from the OpenGL Wiki
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uniform {
    /// Scalars and 1x1 Matrices/Vectors are handled here
    Scalar(UniformKind),
    /// Vectors of a UniformKind
    Vector(UniformKind, UniformSize),
    /// Uniform as a Matrix. Matrix Uniforms can only be `f32`
    Matrix(UniformSize, UniformSize),
}

impl<T> From<T> for Uniform
where
    T: Into<UniformKind>,
{
    fn from(v: T) -> Uniform {
        let kind: UniformKind = v.into();
        Uniform::Scalar(kind)
    }
}

impl Default for Uniform {
    fn default() -> Uniform {
        Uniform::Scalar(UniformKind::default())
    }
}
