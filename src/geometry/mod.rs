
pub mod vector;

use geometry::vector::{Vector2, VectorNumber};

pub trait Geometric<N: VectorNumber>: Sized {
    type Output;
    fn min_extends(&self) -> Vector2<N>;
    fn max_extends(&self) -> Vector2<N>;
    fn is_edge(self, at: Vector2<N>) -> bool;
    fn in_geometric(self, at: Vector2<N>) -> bool;
}
