//! A `rectangle` is a quadrilateral with four right angles

use geometry::{GeometricNum};

#[derive(Debug, Clone, Copy)]
pub struct Rectangle<N: GeometricNum>([N; 4]);

impl<N: GeometricNum> Rectangle<N> {
    /// Creates new Rectangle
    pub fn new(v: [N; 4]) -> Rectangle<N> { return Rectangle(v); }
    /// Returns the X axis of Rectangle's origin
    pub fn get_x(&self) -> N { return self.0[0] }
    /// Returns the Y axis of Rectangle's origin
    pub fn get_y(&self) -> N { return self.0[1] }
    /// Returns the `width` of Rectangle
    pub fn get_width(&self) -> N { return self.0[2] }
    /// Returns the `height` of Rectangle
    pub fn get_height(&self) -> N { return self.0[3] }
    /// Returns array of `N` of rexctangle
    pub fn to_array(&self) -> [N; 4] { return self.0 }

}

impl<N: GeometricNum> PartialEq for Rectangle<N> {
    fn eq(&self, other: &Rectangle<N>) -> bool {
        let ref array = self.0;
        let ref other_array = other.0;
        return array[0] == other_array[0] && array[1] == other_array[1]
            && array[2] == other_array[2] && array[3] == other_array[3];
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;
    use geometry::rectangle::Rectangle;

    #[test]
    fn new() {
        assert_that(&(Rectangle::new([0, 0, 1, 1]))).is_equal_to(Rectangle([0, 0, 1, 1]));
    }

    #[test]
    fn get_x() {
        assert_that(&(Rectangle::new([5, 5, 10, 10]).get_x())).is_equal_to(5);
    }

    #[test]
    fn get_y() {
        assert_that(&(Rectangle::new([5, 7, 10, 10]).get_y())).is_equal_to(7);
    }

    #[test]
    fn get_width() {
        assert_that(&(Rectangle::new([5, 7, 15, 10]).get_width())).is_equal_to(15);
    }

    #[test]
    fn get_height() {
        assert_that(&(Rectangle::new([5, 7, 15, 10]).get_height())).is_equal_to(10);
    }
}
