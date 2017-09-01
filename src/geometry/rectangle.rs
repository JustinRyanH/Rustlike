//! A `rectangle` is a quadrilateral with four right angles
use geometry::vector::Vector2;
use geometry::{Geometric, GeometricNum, collide};

/// `Rectangle` structures an array of origin.x, origin.y, width,
/// and height
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Rectangle<N: GeometricNum>([N; 4]);

impl<N: GeometricNum> Rectangle<N> {
    /// Creates new Rectangle
    pub fn new(v: [N; 4]) -> Rectangle<N> { return Rectangle(v); }
    /// Returns the X axis of Rectangle's origin
    pub fn get_x(&self) -> N { return self.0[0]; }
    /// Returns the Y axis of Rectangle's origin
    pub fn get_y(&self) -> N { return self.0[1]; }
    /// Returns the `width` of Rectangle
    pub fn get_width(&self) -> N { return self.0[2]; }
    /// Returns the `height` of Rectangle
    pub fn get_height(&self) -> N { return self.0[3]; }
    /// Returns array of `N` of recctangle
    pub fn to_array(&self) -> [N; 4] { return self.0; }
    /// Returns zero origin array of rectangle
    pub fn unit_array(&self) -> [N; 4] { [N::zero(), N::zero(), self.get_width(), self.get_height()] }
}

impl<N: GeometricNum> Geometric<N> for Rectangle<N> {
    fn min_extends(&self) -> Vector2<N> {
        return Vector2::new([self.get_x(), self.get_y()]);
    }

    fn max_extends(&self) -> Vector2<N> {
        return self.min_extends() + Vector2::new([self.get_width(), self.get_height()]);
    }
    fn in_geometric(self, at: Vector2<N>) -> bool { return collide(self, at); }
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
    use geometry::Geometric;
    use geometry::vector::Vector2;
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

    #[test]
    fn unit_array() {
        assert_that(&(Rectangle::new([5, 7, 15, 10]).unit_array())).is_equal_to([0, 0, 15, 10])
    }

    #[test]
    fn min_extends() {
        assert_that(&(Rectangle::new([5, 7, 15, 10]).min_extends())).is_equal_to(Vector2::new([5, 7]))
    }

    #[test]
    fn max_extends() {
        assert_that(&(Rectangle::new([5, 7, 15, 10]).max_extends())).is_equal_to(Vector2::new([20, 17]))
    }

    #[test]
    fn in_geometric() {
        let subject = Rectangle::new([5, 7, 15, 10]);
        let is_in_geometric = vec![
            Vector2::new([5, 7]),
            Vector2::new([19, 16]),
            Vector2::new([5, 16]),
            Vector2::new([19, 7]),
            Vector2::new([14, 13]),
        ];

        for edge in &is_in_geometric {
            assert_that(&(subject.in_geometric(*edge))).is_equal_to(true);
        }

        let is_not_in_geometric = vec![
            Vector2::new([4, 7]),
            Vector2::new([5, 6]),
            Vector2::new([20, 17]),
            Vector2::new([5, 17]),
            Vector2::new([20, 7]),
        ];

        for edge in &is_not_in_geometric {
            assert_that(&(subject.in_geometric(*edge))).is_equal_to(false);
        }
    }
}
