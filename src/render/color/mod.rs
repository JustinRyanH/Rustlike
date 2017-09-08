//! Set of common available colors

pub mod list;

/// Basic Matt RGB Color
#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub struct BasicRGB([u8; 3]);

impl BasicRGB {
    /// Creates a new BasicRGB Structure from a array of 0..255
    pub fn new(color: [u8; 3]) -> BasicRGB { BasicRGB(color) }
    /// Returns an array of u8 with 
    pub fn with_alpha(&self, alpha: u8) -> [u8; 4] {
        [self.0[0], self.0[0], self.0[0], alpha]
    }
    /// Returns RGB to an array of floats 0..1 including the given alpha
    pub fn as_float(&self, alpha: u8) -> [f32; 4] {
        let r: f32 = match self.0[0] {
            0 => 0.0,
            _ => self.0[0] as f32 / 255.0, 
        };
        let g: f32 = match self.0[1] {
            0 => 0.0,
            _ => self.0[1] as f32 / 255.0,
        };
        let b: f32 = match self.0[2] {
            0 => 0.0,
            _ => self.0[2] as f32 / 255.0,
        };
        let a: f32 = match alpha {
            0 => 0.0,
            _ => alpha as f32 / 255.0,
        };
        [r, g, b, a]
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;
    use render::color::list::YELLOW;
    
    #[test]
    fn as_float() {
        assert_that(&(YELLOW.as_float(102))).is_equal_to([1.0, 1.0, 0.0, 0.4]);
    }
}
