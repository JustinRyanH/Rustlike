use std::ops::{Add, Sub, Mul, Div};

use num::{Num, One, Zero};

use geometry::Geometric;


pub trait VectorNumber: Num  + One + Zero + PartialEq + Copy {}
impl<T> VectorNumber for T where T: Num  + One + Zero + PartialEq + Copy {}

/// Used to represent 2D Vector
#[derive(Debug, Clone, Copy)]
pub struct Vector2<N: VectorNumber>([N; 2]);

impl<N: VectorNumber> Vector2<N> {
    /// Create new Vector2
    pub fn new(v: [N; 2]) -> Vector2<N> { return Vector2(v) }
    /// Creates a unit Vector2
    pub fn unit() -> Vector2<N> { return Vector2([N::one(), N::one()]) }
    /// Return Vector2 as an Array
    pub fn as_array(&self) -> [N; 2] { return self.0 }
    pub fn get_x(&self) -> N { return self.0[0] }
    pub fn get_y(&self) -> N { return self.0[1] }
}

impl<N: VectorNumber> PartialEq for Vector2<N> {
    fn eq(&self, other: &Vector2<N>) -> bool {
        self.0[0] == other.0[0] && self.0[1] == other.0[1]
    }
}

impl<N: VectorNumber> Add for Vector2<N> {
    type Output = Vector2<N>;

    fn add(self, right: Vector2<N>) -> Vector2<N> {
        return Vector2([self.0[0] + right.0[0], self.0[1] + right.0[1]]);
    }
}

impl<N: VectorNumber> Sub for Vector2<N> {
    type Output = Vector2<N>;

    fn sub(self, right: Vector2<N>) -> Vector2<N> {
        return Vector2([self.0[0] - right.0[0], self.0[1] - right.0[1]]);
    }
}


impl<N: VectorNumber> Mul<N> for Vector2<N> {
    type Output = Vector2<N>;

    fn mul(self, rhs: N) -> Vector2<N> {
        return Vector2([self.0[0] * rhs, self.0[1] * rhs]);
    }
}

impl<N: VectorNumber> Div<N> for Vector2<N> {
    type Output = Vector2<N>;

    fn div(self, rhs: N) -> Vector2<N> {
        return Vector2([self.0[0] / rhs, self.0[1] / rhs]);
    }
}

impl<N: VectorNumber> Geometric<N> for Vector2<N> {
    type Output = Vector2<N>;

    fn min_extends(&self) -> Vector2<N> { Vector2(self.0)}
    fn max_extends(&self) -> Vector2<N> { Vector2(self.0)+Vector2::unit() }
    fn is_edge(self, at: Vector2<N>) -> bool { self == at }
    fn in_geometric(self, at: Vector2<N>) -> bool { self == at }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;
    use geometry::Geometric;
    use geometry::vector::{Vector2};

    #[test]
    fn unit() {
        assert_that(&(Vector2([0.0, 0.0]) + Vector2::unit())).is_equal_to(Vector2([1.0, 1.0]));
        assert_that(&(Vector2([0, 0]) + Vector2::unit())).is_equal_to(Vector2([1, 1]));
    }

    #[test]
    fn partial_eq() {
        assert_that(&Vector2([1, 1])).is_equal_to(&Vector2([1, 1]));
    }

    #[test]
    fn add() {
        assert_that(&(Vector2([-5, 5]) + Vector2([1, 1]))).is_equal_to(&Vector2([-4, 6]));
    }

    #[test]
    fn sub() {
        assert_that(&(Vector2([-5, 5]) - Vector2([1, 1]))).is_equal_to(&Vector2([-6, 4]));
    }

    #[test]
    fn mul_scalar() {
        assert_that(&(Vector2([-5, 5]) * 2)).is_equal_to(&Vector2([-10, 10]));
    }

    #[test]
    fn div_scalar() {
        assert_that(&(Vector2([-10, 10]) / 2)).is_equal_to(&(Vector2([-5, 5])));
        assert_that(&(Vector2([-5, 5]) / -1)).is_equal_to(&(Vector2([5, -5])));
    }

    #[test]
    fn min_extends() {
        let subject = Vector2([4, 10]);
        assert_that(&(subject.min_extends())).is_equal_to(subject);
    }

    #[test]
    fn max_extends() {
        let subject = Vector2([4, 10]);
        assert_that(&(subject.max_extends())).is_equal_to(subject + Vector2::unit());
    }

    #[test]
    fn is_edge() {
        assert_that(&(Vector2([5, 5]).is_edge(Vector2([5, 5])))).is_equal_to(true);
        assert_that(&(Vector2([4, 5]).is_edge(Vector2([5, 5])))).is_equal_to(false);
    }

    #[test]
    fn in_geometric() {
        assert_that(&(Vector2([5, 5]).in_geometric(Vector2([5, 5])))).is_equal_to(true);
        assert_that(&(Vector2([4, 5]).in_geometric(Vector2([5, 5])))).is_equal_to(false);
    }

    #[test]
    fn as_array() {
        assert_that(&(Vector2([10, 15]).as_array())).is_equal_to([10, 15]);
    }
}
