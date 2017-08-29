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
    /// Checks if given `vector` is on edge of geometric
    fn is_edge(self, at: Vector2<N>) -> bool;
    /// Checks if given `vector` is inside given geometric
    fn in_geometric(self, at: Vector2<N>) -> bool;
}

//pub fn collide<N: GeometricNum>(left: Geometric<N>, right: Geometric<N>) -> bool {
//    let axisDistance = right.min_extends() - left.max_extends();
//
//}


