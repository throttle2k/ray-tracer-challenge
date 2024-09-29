use std::ops::{Add, Div, Mul, Neg, Sub};

use approx_eq::ApproxEq;

use super::{points::Point, Tuple};

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn x_norm() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    pub fn y_norm() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn z_norm() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x() * self.x() + self.y() * self.y() + self.z() * self.z())
    }

    pub fn normalize(&self) -> Vector {
        *self / self.magnitude()
    }

    pub fn dot(&self, other: Vector) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn reflect(&self, n: Vector) -> Vector {
        let dp = self.dot(n);
        let double_n = n * 2.0;
        *self - (double_n * dp)
    }
}

impl Tuple for Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }

    fn w(&self) -> f64 {
        0.0
    }

    fn zero() -> Self {
        Vector::new(0.0, 0.0, 0.0)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x.approx_eq(other.x) && self.y.approx_eq(other.y) && self.z.approx_eq(other.z)
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Add<Vector> for &Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Add<&Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Self::Output {
        Vector::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Self::Output {
        Vector::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Add<Point> for &Vector {
    type Output = Vector;

    fn add(self, rhs: Point) -> Self::Output {
        Vector::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Vector::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magnitude_of_1_0_0() {
        let vector = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(vector.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_0_1_0() {
        let vector = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(vector.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_0_0_1() {
        let vector = Vector::new(0.0, 0.0, 1.0);
        assert_eq!(vector.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_1_2_3() {
        let vector = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(vector.magnitude(), f64::sqrt(14.0));
    }

    #[test]
    fn magnitude_of_minus_1_2_3() {
        let vector = Vector::new(-1.0, -2.0, -3.0);
        assert_eq!(vector.magnitude(), f64::sqrt(14.0));
    }

    #[test]
    fn normalize_4_0_0() {
        let vector = Vector::new(4.0, 0.0, 0.0);
        assert_eq!(vector.normalize(), Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn normalize_1_2_3() {
        let vector = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(vector.normalize(), Vector::new(0.26726, 0.53452, 0.80178));
    }

    #[test]
    fn magnitude_of_normalized_vector_is_1() {
        let vector = Vector::new(1.0, 2.0, 3.0);
        let norm = vector.normalize();
        assert_eq!(norm.magnitude(), 1.0);
    }

    #[test]
    fn dot_product_of_two_vector_is_a_scalar() {
        let vector1 = Vector::new(1.0, 2.0, 3.0);
        let vector2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(vector1.dot(vector2), 20.0);
    }

    #[test]
    fn cross_product_of_two_vector_is_a_vector() {
        let vector1 = Vector::new(1.0, 2.0, 3.0);
        let vector2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(vector1 * vector2, Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(vector2 * vector1, Vector::new(1.0, -2.0, 1.0));
    }

    #[test]
    fn reflecting_a_vector_approaching_45_deg() {
        let v = Vector::new(1.0, -1.0, 0.0);
        let n = Vector::y_norm();
        let r = v.reflect(n);
        assert_eq!(r, Vector::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflecting_a_vector_off_a_slanted_surface() {
        let v = Vector::new(0.0, -1.0, 0.0);
        let n = Vector::new(f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0, 0.0);
        let r = v.reflect(n);
        assert_eq!(r, Vector::new(1.0, 0.0, 0.0));
    }
}
