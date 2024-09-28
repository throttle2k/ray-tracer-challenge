use crate::matrix::Matrix;

pub type Transformation = Matrix;

impl Transformation {
    pub fn new_transform() -> Self {
        Matrix::identity(4)
    }

    pub fn translation(&self, x: f64, y: f64, z: f64) -> Self {
        let mut m = Matrix::identity(4);
        m[(0, 3)] = x;
        m[(1, 3)] = y;
        m[(2, 3)] = z;
        &m * self
    }

    pub fn scaling(&self, x: f64, y: f64, z: f64) -> Self {
        let mut m = Matrix::identity(4);
        m[(0, 0)] = x;
        m[(1, 1)] = y;
        m[(2, 2)] = z;
        &m * self
    }

    pub fn reflect_x(&self) -> Self {
        Self::scaling(&self, -1.0, 1.0, 1.0)
    }

    pub fn reflect_y(&self) -> Matrix {
        Self::scaling(&self, 1.0, -1.0, 1.0)
    }

    pub fn reflect_z(&self) -> Matrix {
        Self::scaling(&self, 1.0, 1.0, -1.0)
    }

    pub fn rotation_x(&self, rad: f64) -> Matrix {
        let mut m = Matrix::identity(4);
        m[(1, 1)] = f64::cos(rad);
        m[(1, 2)] = -f64::sin(rad);
        m[(2, 1)] = f64::sin(rad);
        m[(2, 2)] = f64::cos(rad);
        &m * self
    }

    pub fn rotation_y(&self, rad: f64) -> Matrix {
        let mut m = Matrix::identity(4);
        m[(0, 0)] = f64::cos(rad);
        m[(0, 2)] = f64::sin(rad);
        m[(2, 0)] = -f64::sin(rad);
        m[(2, 2)] = f64::cos(rad);
        &m * self
    }

    pub fn rotation_z(&self, rad: f64) -> Matrix {
        let mut m = Matrix::identity(4);
        m[(0, 0)] = f64::cos(rad);
        m[(0, 1)] = -f64::sin(rad);
        m[(1, 0)] = f64::sin(rad);
        m[(1, 1)] = f64::cos(rad);
        &m * self
    }

    pub fn shearing(&self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
        let mut m = Matrix::identity(4);
        m[(0, 1)] = xy;
        m[(0, 2)] = xz;
        m[(1, 0)] = yx;
        m[(1, 2)] = yz;
        m[(2, 0)] = zx;
        m[(2, 1)] = zy;
        &m * self
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{points::Point, tuples::Tuple, vectors::Vector};

    use super::*;

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = Transformation::new_transform().translation(5.0, -3.0, 2.0);
        let p = Point::new(-3.0, 4.0, 5.0);
        assert_eq!(&transform * &p, Point::new(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = Transformation::new_transform().translation(5.0, -3.0, 2.0);
        let inv = transform.inverse().unwrap();
        let p = Point::new(-3.0, 4.0, 5.0);
        assert_eq!(&inv * &p, Point::new(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = Transformation::new_transform().translation(5.0, -3.0, 2.0);
        let v = Vector::new(-3.0, 4.0, 5.0);
        assert_eq!(&transform * &v, v);
    }

    #[test]
    fn scaling_a_point() {
        let transform = Transformation::new_transform().scaling(2.0, 3.0, 4.0);
        let p = Point::new(-4.0, 6.0, 8.0);
        assert_eq!(&transform * &p, Point::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_a_vector() {
        let transform = Transformation::new_transform().scaling(2.0, 3.0, 4.0);
        let v = Vector::new(-4.0, 6.0, 8.0);
        assert_eq!(&transform * &v, Vector::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = Transformation::new_transform().scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse().unwrap();
        let v = Vector::new(-4.0, 6.0, 8.0);
        assert_eq!(&inv * &v, Vector::new(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_is_scaling_by_negative_value() {
        let transform = Transformation::new_transform().reflect_x();
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(&transform * &p, Point::new(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_a_point_around_x_axis() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Transformation::new_transform().rotation_x(PI / 4.0);
        let full_quarter = Transformation::new_transform().rotation_x(PI / 2.0);
        assert_eq!(
            &half_quarter * &p,
            Point::new(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0)
        );
        assert_eq!(&full_quarter * &p, Point::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn inverse_of_x_rotation_rotates_in_the_opposite_direction() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Transformation::new_transform().rotation_x(PI / 4.0);
        let inv = half_quarter.inverse().unwrap();
        assert_eq!(
            &inv * &p,
            Point::new(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0)
        );
    }

    #[test]
    fn rotating_a_point_around_y_axis() {
        let p = Point::new(0.0, 0.0, 1.0);
        let half_quarter = Transformation::new_transform().rotation_y(PI / 4.0);
        let full_quarter = Transformation::new_transform().rotation_y(PI / 2.0);
        assert_eq!(
            &half_quarter * &p,
            Point::new(f64::sqrt(2.0) / 2.0, 0.0, f64::sqrt(2.0) / 2.0)
        );
        assert_eq!(&full_quarter * &p, Point::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_a_point_around_z_axis() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Transformation::new_transform().rotation_z(PI / 4.0);
        let full_quarter = Transformation::new_transform().rotation_z(PI / 2.0);
        assert_eq!(
            &half_quarter * &p,
            Point::new(-f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0, 0.0)
        );
        assert_eq!(&full_quarter * &p, Point::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_y() {
        let transform = Transformation::new_transform().shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(&transform * &p, Point::new(5.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_z() {
        let transform = Transformation::new_transform().shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(&transform * &p, Point::new(6.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_x() {
        let transform = Transformation::new_transform().shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(&transform * &p, Point::new(2.0, 5.0, 4.0));
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_z() {
        let transform = Transformation::new_transform().shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(&transform * &p, Point::new(2.0, 7.0, 4.0));
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_x() {
        let transform = Transformation::new_transform().shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(&transform * &p, Point::new(2.0, 3.0, 6.0));
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_y() {
        let transform = Transformation::new_transform().shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(&transform * &p, Point::new(2.0, 3.0, 7.0));
    }

    #[test]
    fn chaining_transformations() {
        let p = Point::new(1.0, 0.0, 1.0);
        let transform = Transformation::new_transform()
            .rotation_x(PI / 2.0)
            .scaling(5.0, 5.0, 5.0)
            .translation(10.0, 5.0, 7.0);
        assert_eq!(&transform * &p, Point::new(15.0, 0.0, 7.0));
    }
}
