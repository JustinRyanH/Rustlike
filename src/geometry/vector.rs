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


impl<N: Num + PartialEq + Copy> Mul<N> for Vector2<N> {
    type Output = Vector2<N>;

    fn mul(self, rhs: N) -> Vector2<N> {
        return Vector2([self.0[0] * rhs, self.0[1] * rhs]);
    }
}

impl<N: Num + PartialEq + Copy> Div<N> for Vector2<N> {
    type Output = Vector2<N>;

    fn div(self, rhs: N) -> Vector2<N> {
        return Vector2([self.0[0] / rhs, self.0[1] / rhs]);
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
        assert_that(&(Vector2([-5, 5]) * 2)).is_equal_to(&Vector2([-10, 10]));
    }

    #[test]
    fn div_scalar() {
        assert_that(&(Vector2([-10, 10]) / 2)).is_equal_to(&(Vector2([-5, 5])));
        assert_that(&(Vector2([-5, 5]) / -1)).is_equal_to(&(Vector2([5, -5])));
    }
}
