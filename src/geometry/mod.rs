
pub mod vector;

use num::Num;
use geometry::vector::Vector2;

pub trait Geometric<N: Num + Copy + PartialEq>: Sized {
    type Output;
    fn min_extends(&self) -> Vector2<N>;
    fn max_extends(&self) -> Vector2<N>;
    fn is_edge(self, at: Vector2<N>) -> bool;
    fn in_geometric(self, at: Vector2<N>) -> bool;
}
