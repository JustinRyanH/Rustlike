use std::ops::{Add, Sub, Mul, Div};
use num::Num;

#[derive(Debug)]
pub struct Vector2<N: Num + PartialEq + Copy>([N; 2]);


impl<N: Num + PartialEq + Copy> PartialEq for Vector2<N> {
    fn eq(&self, other: &Vector2<N>) -> bool {
        self.0[0] == other.0[0] && self.0[1] == other.0[1]
    }
}

impl<N: Num + PartialEq + Copy> Add for Vector2<N> {
    type Output = Vector2<N>;

    fn add(self, right: Vector2<N>) -> Vector2<N> {
        return Vector2([self.0[0] + right.0[0], self.0[1] + right.0[1]]);
    }
}

impl<N: Num + PartialEq + Copy> Sub for Vector2<N> {
    type Output = Vector2<N>;

    fn sub(self, right: Vector2<N>) -> Vector2<N> {
        return Vector2([self.0[0] - right.0[0], self.0[1] - right.0[1]]);
    }
}
pub struct Scalar<N: Num + PartialEq + Copy>(N);


impl<N: Num + PartialEq + Copy> Mul<Scalar<N>> for Vector2<N> {
    type Output = Vector2<N>;

    fn mul(self, rhs: Scalar<N>) -> Vector2<N> {
        return Vector2([self.0[0] * rhs.0, self.0[1] * rhs.0]);
    }
}

impl<N: Num + PartialEq + Copy> Mul<Vector2<N>> for Scalar<N> {
    type Output = Vector2<N>;

    fn mul(self, rhs: Vector2<N>) -> Vector2<N> {
        return rhs * self;
    }
}

impl<N: Num + PartialEq + Copy> Div<Scalar<N>> for Vector2<N> {
    type Output = Vector2<N>;

    fn div(self, rhs: Scalar<N>) -> Vector2<N> {
        return Vector2([self.0[0] / rhs.0, self.0[1] / rhs.0]);
    }
}


#[cfg(test)]
mod tests {
    use spectral::prelude::*;
    use geometry::vector::{Vector2, Scalar};

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
        assert_that(&(Vector2([-5, 5]) * Scalar(2))).is_equal_to(&Vector2([-10, 10]));
    }

    #[test]
    fn mul_scalar_commutative() {
        assert_that(&(Vector2([-5, 5]) * Scalar(2))).is_equal_to(&(Scalar(2) * Vector2([-5, 5])))
    }

    #[test]
    fn div_scalar() {
        assert_that(&(Vector2([-10, 10]) / Scalar(2))).is_equal_to(&(Vector2([-5, 5])));
        assert_that(&(Vector2([-5, 5]) / Scalar(-1))).is_equal_to(&(Vector2([5, -5])));
    }

}
