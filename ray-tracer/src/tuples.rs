pub trait Tuple {
    fn new(x: f64, y: f64, z: f64) -> Self;
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn w(&self) -> f64;

    fn zero() -> Self;
}

#[cfg(test)]
mod tests {
    use crate::tuples::Tuple;

    use crate::{points::Point, vectors::Vector};

    #[test]
    fn a_point_is_a_tuple_with_w_1() {
        let point = Point::new(4.3, -4.2, 3.1);
        assert_eq!(point.w(), 1.0);
    }

    #[test]
    fn a_vector_is_a_tuple_with_w_0() {
        let vector = Vector::new(4.3, -4.2, 3.1);
        assert_eq!(vector.w(), 0.0);
    }

    #[test]
    fn summing_a_point_to_a_vector_returns_a_point() {
        let point = Point::new(3.0, -2.0, 5.0);
        let vector = Vector::new(-2.0, 3.0, 1.0);
        assert_eq!(point + vector, Point::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn subtracting_two_points_returns_vector() {
        let point1 = Point::new(3.0, 2.0, 1.0);
        let point2 = Point::new(5.0, 6.0, 7.0);
        assert_eq!(point1 - point2, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_a_vector_from_a_point_returns_vector() {
        let point = Point::new(-3.0, -2.0, -1.0);
        let vector = Vector::new(5.0, 6.0, 7.0);
        assert_eq!(point - vector, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_vectors_returns_a_vector() {
        let vector1 = Vector::new(3.0, 2.0, 1.0);
        let vector2 = Vector::new(5.0, 6.0, 7.0);
        assert_eq!(vector1 - vector2, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn negating_a_vector_returns_a_vector() {
        let vector = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(-vector, Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn multiplying_a_vector_for_a_scalar_returns_a_vector() {
        let vector = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(vector * 3.5, Vector::new(3.5, -7.0, 10.5));
    }

    #[test]
    fn dividing_a_vector_by_a_scalar_returns_a_vector() {
        let vector = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(vector / 2.0, Vector::new(0.5, -1.0, 1.5));
    }
}
