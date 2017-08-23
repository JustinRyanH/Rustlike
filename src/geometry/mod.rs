//! Geometry represents geometric elements and the space they
//! consume in the world
pub mod vector;

use geometry::vector::{Vector2, VectorNumber};

/// Geometric is used to test information about a geometric object
pub trait Geometric<N: VectorNumber>: Sized {
    /// Returns closest point of geometric to origin
    fn min_extends(&self) -> Vector2<N>;
    /// Returns farthest reaches of geometric from origin
    fn max_extends(&self) -> Vector2<N>;
    /// Checks if given Vector is on edge of geometric
    fn is_edge(self, at: Vector2<N>) -> bool;
    /// Checks if given Vector is inside given geometric
    fn in_geometric(self, at: Vector2<N>) -> bool;
}
