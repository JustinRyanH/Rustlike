use std::ops::{Add, Sub, Mul, Div};

use num::{Num, One, Zero};

use geometry::Geometric;

#[derive(Debug)]
pub struct Vector2<N: Num  + One + Zero + PartialEq + Copy>([N; 2]);

impl<N: Num  + One + Zero + PartialEq + Copy> Vector2<N> {
    fn unit() -> Vector2<N> {
        return Vector2([N::one(), N::one()])
    }
}

impl<N: Num  + One + Zero + PartialEq + Copy> PartialEq for Vector2<N> {
    fn eq(&self, other: &Vector2<N>) -> bool {
        self.0[0] == other.0[0] && self.0[1] == other.0[1]
    }
}

impl<N: Num  + One + Zero + PartialEq + Copy> Add for Vector2<N> {
    type Output = Vector2<N>;

    fn add(self, right: Vector2<N>) -> Vector2<N> {
        return Vector2([self.0[0] + right.0[0], self.0[1] + right.0[1]]);
    }
}

impl<N: Num  + One + Zero + PartialEq + Copy> Sub for Vector2<N> {
    type Output = Vector2<N>;

    fn sub(self, right: Vector2<N>) -> Vector2<N> {
        return Vector2([self.0[0] - right.0[0], self.0[1] - right.0[1]]);
    }
}


impl<N: Num  + One + Zero + PartialEq + Copy> Mul<N> for Vector2<N> {
    type Output = Vector2<N>;

    fn mul(self, rhs: N) -> Vector2<N> {
        return Vector2([self.0[0] * rhs, self.0[1] * rhs]);
    }
}

impl<N: Num  + One + Zero + PartialEq + Copy> Div<N> for Vector2<N> {
    type Output = Vector2<N>;

    fn div(self, rhs: N) -> Vector2<N> {
        return Vector2([self.0[0] / rhs, self.0[1] / rhs]);
    }
}

impl<N: Num  + One + Zero + PartialEq + Copy> Geometric<N> for Vector2<N> {
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
}
