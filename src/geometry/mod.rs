//! Geometry represents geometric elements and the space they
//! consume in the world

pub mod vector;
pub mod rectangle;

use geometry::vector::{Vector2};
use num::{Num, One, Zero};

/// `GeometricNum` are types of numeric measurements for
/// axis of an `Geometric` object. They must be a number a
/// `number` that has a `zero` and a `one` value. It can be
/// either `Float` or an `Integer` or various sizes
pub trait GeometricNum: Num  + One + Zero + PartialEq + PartialOrd + Copy {}
impl<N> GeometricNum for N where N: Num  + One + Zero + PartialEq + PartialOrd + Copy {}

/// `Geometric` is standard traits from all components that take up
/// space.
pub trait Geometric<N: GeometricNum>: Sized {
    /// Returns closest point of geometric to origin
    fn min_extends(&self) -> Vector2<N>;
    /// Returns farthest reaches of geometric from origin
    fn max_extends(&self) -> Vector2<N>;
    /// Checks if given `vector` is inside given geometric
    fn in_geometric(self, at: Vector2<N>) -> bool;
}

/// Compared two different Geometric shapes and returns `true` if they overlap and
/// false if they do not
pub fn collide<N: GeometricNum, L: Geometric<N>, R: Geometric<N>>(left: L, right: R) -> bool {
    return (right.min_extends() - left.max_extends())
        .max(left.min_extends() - right.max_extends())
        .largest_axis() < N::zero();
}


#[cfg(test)]
mod tests {
    use spectral::prelude::*;
    use geometry::{collide};
    use geometry::vector::Vector2;
    use geometry::rectangle::Rectangle;

    #[test]
    fn collide_vector() {
        assert_that(&(collide(Vector2::new([5, 5]), Vector2::new([5, 5])))).is_equal_to(true);
        assert_that(&(collide(Vector2::new([5, 5]), Vector2::new([4, 4])))).is_equal_to(false);
        assert_that(&(collide(Vector2::new([5, 5]), Vector2::new([6, 6])))).is_equal_to(false);
    }

    #[test]
    fn collide_rect() {
        assert_that(&(collide(Rectangle::new([5, 5, 10, 15]), Vector2::new([5, 5])))).is_equal_to(true);
        assert_that(&(collide(Rectangle::new([5, 5, 10, 15]), Vector2::new([4, 4])))).is_equal_to(false);
        assert_that(&(collide(Rectangle::new([5, 5, 10, 15]), Vector2::new([6, 6])))).is_equal_to(true);
        assert_that(&(collide(Rectangle::new([5, 5, 10, 15]), Vector2::new([14, 19])))).is_equal_to(true);
        assert_that(&(collide(Rectangle::new([5, 5, 10, 15]), Vector2::new([14, 10])))).is_equal_to(true);
        assert_that(&(collide(Rectangle::new([5, 5, 10, 15]), Rectangle::new([14, 10, 5, 5])))).is_equal_to(true);
    }
}
