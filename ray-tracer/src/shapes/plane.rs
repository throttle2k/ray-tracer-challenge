use approx_eq::EPSILON;

use crate::{
    bounds::Bounds,
    rays::Ray,
    tuples::{points::Point, vectors::Vector, Tuple},
};

pub struct Plane {}

impl Plane {
    pub fn normal_at(_object_point: Point) -> Vector {
        return Vector::y_norm();
    }

    pub fn intersects(ray: Ray) -> Vec<f64> {
        let mut intersections = Vec::new();
        if ray.direction.y().abs() > EPSILON {
            let t = -ray.origin.y() / ray.direction.y();
            intersections.push(t);
        }
        intersections
    }

    pub fn bounds() -> crate::bounds::Bounds {
        Bounds::new(
            Point::new(f64::NEG_INFINITY, 0.0, f64::NEG_INFINITY),
            Point::new(f64::INFINITY, 0.0, f64::INFINITY),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::tuples::Tuple;

    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let n1 = Plane::normal_at(Point::zero());
        let n2 = Plane::normal_at(Point::new(10.0, 0.0, -10.0));
        let n3 = Plane::normal_at(Point::new(-5.0, 0.0, 150.0));
        assert_eq!(n1, Vector::y_norm());
        assert_eq!(n2, Vector::y_norm());
        assert_eq!(n3, Vector::y_norm());
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let r = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::z_norm());
        let xs = Plane::intersects(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::z_norm());
        let xs = Plane::intersects(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let r = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
        let xs = Plane::intersects(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0], 1.0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let r = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let xs = Plane::intersects(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0], 1.0);
    }
}
