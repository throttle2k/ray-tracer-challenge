use crate::{
    matrix::Matrix,
    tuples::{points::Point, vectors::Vector, Tuple},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Transformation {
    pub matrix: Matrix,
    inverse: Option<Matrix>,
    inverse_transposed: Option<Matrix>,
}

impl Transformation {
    pub fn new_transform() -> Self {
        let matrix = Matrix::identity(4);
        let (inverse, inverse_transposed) = Self::prepare_transform(&matrix);
        Self {
            matrix,
            inverse,
            inverse_transposed,
        }
    }

    fn prepare_transform(t: &Matrix) -> (Option<Matrix>, Option<Matrix>) {
        let transform_inverse = t.inverse();
        let transform_inverse_transpose = if let Some(ti) = &transform_inverse {
            Some(ti.transpose())
        } else {
            None
        };
        (transform_inverse, transform_inverse_transpose)
    }

    pub fn apply_transform(&mut self, t: &Transformation) {
        self.matrix = &t.matrix * &self.matrix;
        let (inverse, inverse_transposed) = Self::prepare_transform(&self.matrix);
        self.inverse = inverse;
        self.inverse_transposed = inverse_transposed;
    }

    pub fn view_transform(from: Point, to: Point, up: Vector) -> Self {
        let forward = (to - from).normalize();
        let up_normalized = up.normalize();
        let left = forward * up_normalized;
        let true_up = left * forward;
        let orientation = Matrix::new(vec![
            vec![left.x(), left.y(), left.z(), 0.0],
            vec![true_up.x(), true_up.y(), true_up.z(), 0.0],
            vec![-forward.x(), -forward.y(), -forward.z(), 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);
        let matrix = &orientation
            * &Transformation::new_transform()
                .translation(-from.x(), -from.y(), -from.z())
                .matrix;
        let (inverse, inverse_transposed) = Self::prepare_transform(&matrix);
        Self {
            matrix,
            inverse,
            inverse_transposed,
        }
    }

    pub fn translation(&self, x: f64, y: f64, z: f64) -> Self {
        let mut m = Matrix::identity(4);
        m[(0, 3)] = x;
        m[(1, 3)] = y;
        m[(2, 3)] = z;
        let t = &m * &self.matrix;
        let (inverse, inverse_transposed) = Self::prepare_transform(&t);
        Self {
            matrix: t,
            inverse,
            inverse_transposed,
        }
    }

    pub fn scaling(&self, x: f64, y: f64, z: f64) -> Self {
        let mut m = Matrix::identity(4);
        m[(0, 0)] = x;
        m[(1, 1)] = y;
        m[(2, 2)] = z;
        let t = &m * &self.matrix;
        let (inverse, inverse_transposed) = Self::prepare_transform(&t);
        Self {
            matrix: t,
            inverse,
            inverse_transposed,
        }
    }

    pub fn reflect_x(&self) -> Self {
        Self::scaling(&self, -1.0, 1.0, 1.0)
    }

    pub fn reflect_y(&self) -> Self {
        Self::scaling(&self, 1.0, -1.0, 1.0)
    }

    pub fn reflect_z(&self) -> Self {
        Self::scaling(&self, 1.0, 1.0, -1.0)
    }

    pub fn rotation_x(&self, rad: f64) -> Self {
        let mut m = Matrix::identity(4);
        m[(1, 1)] = f64::cos(rad);
        m[(1, 2)] = -f64::sin(rad);
        m[(2, 1)] = f64::sin(rad);
        m[(2, 2)] = f64::cos(rad);
        let t = &m * &self.matrix;
        let (inverse, inverse_transposed) = Self::prepare_transform(&t);
        Self {
            matrix: t,
            inverse,
            inverse_transposed,
        }
    }

    pub fn rotation_y(&self, rad: f64) -> Self {
        let mut m = Matrix::identity(4);
        m[(0, 0)] = f64::cos(rad);
        m[(0, 2)] = f64::sin(rad);
        m[(2, 0)] = -f64::sin(rad);
        m[(2, 2)] = f64::cos(rad);
        let t = &m * &self.matrix;
        let (inverse, inverse_transposed) = Self::prepare_transform(&t);
        Self {
            matrix: t,
            inverse,
            inverse_transposed,
        }
    }

    pub fn rotation_z(&self, rad: f64) -> Self {
        let mut m = Matrix::identity(4);
        m[(0, 0)] = f64::cos(rad);
        m[(0, 1)] = -f64::sin(rad);
        m[(1, 0)] = f64::sin(rad);
        m[(1, 1)] = f64::cos(rad);
        let t = &m * &self.matrix;
        let (inverse, inverse_transposed) = Self::prepare_transform(&t);
        Self {
            matrix: t,
            inverse,
            inverse_transposed,
        }
    }

    pub fn shearing(&self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        let mut m = Matrix::identity(4);
        m[(0, 1)] = xy;
        m[(0, 2)] = xz;
        m[(1, 0)] = yx;
        m[(1, 2)] = yz;
        m[(2, 0)] = zx;
        m[(2, 1)] = zy;
        let t = &m * &self.matrix;
        let (inverse, inverse_transposed) = Self::prepare_transform(&t);
        Self {
            matrix: t,
            inverse,
            inverse_transposed,
        }
    }

    pub fn inverse(&self) -> Option<&Matrix> {
        self.inverse.as_ref()
    }

    pub fn inverse_transposed(&self) -> Option<&Matrix> {
        self.inverse_transposed.as_ref()
    }
}

impl From<Matrix> for Transformation {
    fn from(matrix: Matrix) -> Self {
        let (inverse, inverse_transposed) = Self::prepare_transform(&matrix);
        Self {
            matrix,
            inverse,
            inverse_transposed,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{tuples::points::Point, tuples::vectors::Vector, tuples::Tuple};

    use super::*;

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = Transformation::new_transform().translation(5.0, -3.0, 2.0);
        let p = Point::new(-3.0, 4.0, 5.0);
        assert_eq!(&transform.matrix * &p, Point::new(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = Transformation::new_transform().translation(5.0, -3.0, 2.0);
        let inv = transform.inverse().unwrap();
        let p = Point::new(-3.0, 4.0, 5.0);
        assert_eq!(inv * &p, Point::new(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = Transformation::new_transform().translation(5.0, -3.0, 2.0);
        let v = Vector::new(-3.0, 4.0, 5.0);
        assert_eq!(&transform.matrix * &v, v);
    }

    #[test]
    fn scaling_a_point() {
        let transform = Transformation::new_transform().scaling(2.0, 3.0, 4.0);
        let p = Point::new(-4.0, 6.0, 8.0);
        assert_eq!(&transform.matrix * &p, Point::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_a_vector() {
        let transform = Transformation::new_transform().scaling(2.0, 3.0, 4.0);
        let v = Vector::new(-4.0, 6.0, 8.0);
        assert_eq!(&transform.matrix * &v, Vector::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = Transformation::new_transform().scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse().unwrap();
        let v = Vector::new(-4.0, 6.0, 8.0);
        assert_eq!(inv * &v, Vector::new(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_is_scaling_by_negative_value() {
        let transform = Transformation::new_transform().reflect_x();
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(&transform.matrix * &p, Point::new(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_a_point_around_x_axis() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Transformation::new_transform().rotation_x(PI / 4.0);
        let full_quarter = Transformation::new_transform().rotation_x(PI / 2.0);
        assert_eq!(
            &half_quarter.matrix * &p,
            Point::new(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0)
        );
        assert_eq!(&full_quarter.matrix * &p, Point::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn inverse_of_x_rotation_rotates_in_the_opposite_direction() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Transformation::new_transform().rotation_x(PI / 4.0);
        let inv = half_quarter.inverse().unwrap();
        assert_eq!(
            inv * &p,
            Point::new(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0)
        );
    }

    #[test]
    fn rotating_a_point_around_y_axis() {
        let p = Point::new(0.0, 0.0, 1.0);
        let half_quarter = Transformation::new_transform().rotation_y(PI / 4.0);
        let full_quarter = Transformation::new_transform().rotation_y(PI / 2.0);
        assert_eq!(
            &half_quarter.matrix * &p,
            Point::new(f64::sqrt(2.0) / 2.0, 0.0, f64::sqrt(2.0) / 2.0)
        );
        assert_eq!(&full_quarter.matrix * &p, Point::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_a_point_around_z_axis() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Transformation::new_transform().rotation_z(PI / 4.0);
        let full_quarter = Transformation::new_transform().rotation_z(PI / 2.0);
        assert_eq!(
            &half_quarter.matrix * &p,
            Point::new(-f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0, 0.0)
        );
        assert_eq!(&full_quarter.matrix * &p, Point::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_y() {
        let transform = Transformation::new_transform().shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(&transform.matrix * &p, Point::new(5.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_z() {
        let transform = Transformation::new_transform().shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(&transform.matrix * &p, Point::new(6.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_x() {
        let transform = Transformation::new_transform().shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(&transform.matrix * &p, Point::new(2.0, 5.0, 4.0));
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_z() {
        let transform = Transformation::new_transform().shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(&transform.matrix * &p, Point::new(2.0, 7.0, 4.0));
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_x() {
        let transform = Transformation::new_transform().shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(&transform.matrix * &p, Point::new(2.0, 3.0, 6.0));
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_y() {
        let transform = Transformation::new_transform().shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(&transform.matrix * &p, Point::new(2.0, 3.0, 7.0));
    }

    #[test]
    fn chaining_transformations() {
        let p = Point::new(1.0, 0.0, 1.0);
        let transform = Transformation::new_transform()
            .rotation_x(PI / 2.0)
            .scaling(5.0, 5.0, 5.0)
            .translation(10.0, 5.0, 7.0);
        assert_eq!(&transform.matrix * &p, Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn transformation_matrix_for_default_orientation() {
        let from = Point::zero();
        let to = Point::new(0.0, 0.0, -1.0);
        let up = Vector::y_norm();
        let t = Transformation::view_transform(from, to, up);
        assert_eq!(t.matrix, Matrix::identity(4));
    }

    #[test]
    fn a_view_transformation_matrix_looking_in_positive_z_direction() {
        let from = Point::zero();
        let to = Point::new(0.0, 0.0, 1.0);
        let up = Vector::y_norm();
        let t = Transformation::view_transform(from, to, up);
        assert_eq!(t, Transformation::new_transform().scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn the_view_transformation_moves_the_world() {
        let from = Point::new(0.0, 0.0, 8.0);
        let to = Point::zero();
        let up = Vector::y_norm();
        let t = Transformation::view_transform(from, to, up);
        assert_eq!(
            t,
            Transformation::new_transform().translation(0.0, 0.0, -8.0)
        );
    }

    #[test]
    fn an_arbitrary_view_transformation() {
        let from = Point::new(1.0, 3.0, 2.0);
        let to = Point::new(4.0, -2.0, 8.0);
        let up = Vector::new(1.0, 1.0, 0.0);
        let t = Transformation::view_transform(from, to, up);
        assert_eq!(
            t.matrix,
            Matrix::new(vec![
                vec![-0.50709, 0.50709, 0.67612, -2.36643],
                vec![0.76772, 0.60609, 0.12122, -2.82843],
                vec![-0.35857, 0.59761, -0.71714, 0.00000],
                vec![0.00000, 0.00000, 0.00000, 1.00000]
            ])
        );
    }
}
