//! A `vector` is a representation of point of physical space
//! in this case it represents the distances of each axis to the
//! point

use std::ops::{Add, Sub, Mul, Div};

use geometry::{Geometric, GeometricNum};

fn max<N: PartialOrd>(left: N, right: N) -> N {
    if left > right { return left; }
    return right;
}

fn min<N: PartialOrd>(left: N, right: N) -> N {
    if left > right { return right; }
    return left;
}

/// Used to represent 2D Vector
#[derive(Serialize, Deserialize, Hash, Debug, Clone, Copy)]
pub struct Vector2<N: GeometricNum>([N; 2]);

impl<N: GeometricNum> Vector2<N> {
    /// Create new Vector2
    pub fn new(v: [N; 2]) -> Vector2<N> { return Vector2(v); }
    /// Creates a unit Vector2
    pub fn unit() -> Vector2<N> { return Vector2([N::one(), N::one()]); }

    /// Return Vector2 as an Array
    pub fn as_array(&self) -> [N; 2] { return self.0; }
    /// Returns the X axis of the 2D Vector
    pub fn get_x(&self) -> N { return self.0[0]; }
    /// Returns the Y axis of the 2D Vector
    pub fn get_y(&self) -> N { return self.0[1]; }
    /// Returns new Vector with the biggest value of each axis
    pub fn max(&self, other: Vector2<N>) -> Vector2<N> {
        return Vector2::new([
            max(self.get_x(), other.get_x()),
            max(self.get_y(), other.get_y())
        ]);
    }
    /// Returns new Vector with the smallest value of each axis
    pub fn min(&self, other: Vector2<N>) -> Vector2<N> {
        return Vector2::new([
            min(self.get_x(), other.get_x()),
            min(self.get_y(), other.get_y())
        ]);
    }
    /// Returns the largest axis of a vector
    pub fn largest_axis(&self) -> N { max(self.get_x(), self.get_y()) }
    /// Returns the smallest axis of a vector
    pub fn smallest_axis(&self) -> N { min(self.get_x(), self.get_y()) }
}

impl<N: GeometricNum> PartialEq for Vector2<N> {
    fn eq(&self, other: &Vector2<N>) -> bool {
        self.0[0] == other.0[0] && self.0[1] == other.0[1]
    }
}

impl<N: GeometricNum> Add for Vector2<N> {
    type Output = Vector2<N>;

    fn add(self, right: Vector2<N>) -> Vector2<N> {
        return Vector2([self.0[0] + right.0[0], self.0[1] + right.0[1]]);
    }
}

impl<N: GeometricNum> Sub for Vector2<N> {
    type Output = Vector2<N>;

    fn sub(self, right: Vector2<N>) -> Vector2<N> {
        return Vector2([self.0[0] - right.0[0], self.0[1] - right.0[1]]);
    }
}


impl<N: GeometricNum> Mul<N> for Vector2<N> {
    type Output = Vector2<N>;

    fn mul(self, rhs: N) -> Vector2<N> {
        return Vector2([self.0[0] * rhs, self.0[1] * rhs]);
    }
}

impl<N: GeometricNum> Div<N> for Vector2<N> {
    type Output = Vector2<N>;

    fn div(self, rhs: N) -> Vector2<N> {
        return Vector2([self.0[0] / rhs, self.0[1] / rhs]);
    }
}

impl<N: GeometricNum> Geometric<N> for Vector2<N> {
    fn min_extends(&self) -> Vector2<N> { Vector2(self.0) }
    fn max_extends(&self) -> Vector2<N> { Vector2(self.0) + Vector2::unit() }
    fn in_geometric(self, at: Vector2<N>) -> bool { self == at }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;
    use geometry::Geometric;
    use geometry::vector::Vector2;

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
    fn in_geometric() {
        assert_that(&(Vector2([5, 5]).in_geometric(Vector2([5, 5])))).is_equal_to(true);
        assert_that(&(Vector2([4, 5]).in_geometric(Vector2([5, 5])))).is_equal_to(false);
    }

    #[test]
    fn as_array() {
        assert_that(&(Vector2([10, 15]).as_array())).is_equal_to([10, 15]);
    }

    #[test]
    fn max() {
        assert_that(&(Vector2([10, -5]).max(Vector2([15, -7])))).is_equal_to(Vector2([15, -5]))
    }

    #[test]
    fn min() {
        assert_that(&(Vector2([10.0, -5.0]).min(Vector2([15.0, -7.0])))).is_equal_to(Vector2([10.0, -7.0]))
    }

    #[test]
    fn largest_axis() {
        assert_that(&(Vector2([10.0, -5.0]).largest_axis())).is_equal_to(10.0)
    }

    #[test]
    fn smallest_axis() {
        assert_that(&(Vector2([10.0, -5.0]).smallest_axis())).is_equal_to(-5.0)
    }
}
