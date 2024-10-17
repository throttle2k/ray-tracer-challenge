use approx_eq::EPSILON;

use crate::{
    bounds::Bounds,
    intersections::{Intersection, Intersections},
    rays::Ray,
    tuples::{points::Point, vectors::Vector, Tuple},
};

use super::Object;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Plane {}

impl Plane {
    pub fn normal_at(&self, _object_point: Point) -> Vector {
        return Vector::y_norm();
    }

    pub fn intersects<'a>(&self, object: &'a Object, ray: &Ray) -> Intersections<'a> {
        let mut intersections = Intersections::new();
        if ray.direction.y().abs() > EPSILON {
            let t = -ray.origin.y() / ray.direction.y();
            intersections.push(Intersection::new(t, object));
        }
        intersections
    }

    pub fn bounds(&self) -> Bounds {
        Bounds::new(
            Point::new(f64::NEG_INFINITY, 0.0, f64::NEG_INFINITY),
            Point::new(f64::INFINITY, 0.0, f64::INFINITY),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{shapes::ObjectBuilder, tuples::Tuple};

    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let n1 = Plane::normal_at(&Plane::default(), Point::zero());
        let n2 = Plane::normal_at(&Plane::default(), Point::new(10.0, 0.0, -10.0));
        let n3 = Plane::normal_at(&Plane::default(), Point::new(-5.0, 0.0, 150.0));
        assert_eq!(n1, Vector::y_norm());
        assert_eq!(n2, Vector::y_norm());
        assert_eq!(n3, Vector::y_norm());
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = ObjectBuilder::new_plane().build();
        let r = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::z_norm());
        let xs = p.intersects(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = ObjectBuilder::new_plane().build();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::z_norm());
        let xs = p.intersects(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = ObjectBuilder::new_plane().build();
        let r = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
        let xs = p.intersects(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = ObjectBuilder::new_plane().build();
        let r = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let xs = p.intersects(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
    }

    #[test]
    fn a_plane_has_a_bounding_box() {
        let s = Plane::default();
        let b = s.bounds();
        assert!(b.min().x().is_infinite());
        assert!(b.min().x().is_sign_negative());
        assert_eq!(b.min().y(), 0.0);
        assert!(b.min().z().is_infinite());
        assert!(b.min().z().is_sign_negative());
        assert!(b.max().x().is_infinite());
        assert!(b.max().x().is_sign_positive());
        assert_eq!(b.min().y(), 0.0);
        assert!(b.max().z().is_infinite());
        assert!(b.max().z().is_sign_positive());
    }
}
