use crate::{
    bounds::Bounds,
    intersections::{Intersection, Intersections},
    rays::Ray,
    tuples::{points::Point, vectors::Vector, Tuple},
};

use super::Object;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Sphere {}

impl Sphere {
    pub fn intersects<'a>(&self, object: &'a Object, ray: &Ray) -> Intersections<'a> {
        let sphere_to_ray = ray.origin - Point::new(0.0, 0.0, 0.0);
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            Intersections::new()
        } else {
            let mut xs = Intersections::new();
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            xs.push(Intersection::new(t1, object));
            xs.push(Intersection::new(t2, object));
            xs
        }
    }

    pub fn bounds(&self) -> Bounds {
        Bounds::new(Point::new(-1.0, -1.0, -1.0), Point::new(1.0, 1.0, 1.0))
    }

    pub fn normal_at(&self, object_point: Point) -> Vector {
        object_point - Point::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_of_sphere_at_point_on_x_axis() {
        let n = Sphere::default().normal_at(Point::new(1.0, 0.0, 0.0));
        assert_eq!(n, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_of_sphere_at_point_on_y_axis() {
        let n = Sphere::default().normal_at(Point::new(0.0, 1.0, 0.0));
        assert_eq!(n, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_of_sphere_at_point_on_z_axis() {
        let n = Sphere::default().normal_at(Point::new(0.0, 0.0, 1.0));
        assert_eq!(n, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_of_sphere_at_a_non_axial_point() {
        let n = Sphere::default().normal_at(Point::new(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        ));
        assert_eq!(
            n,
            Vector::new(
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
            )
        );
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let n = Sphere::default().normal_at(Point::new(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        ));
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn a_sphere_has_a_bounding_box() {
        let s = Sphere::default();
        let b = s.bounds();
        assert_eq!(b.min(), &Point::new(-1.0, -1.0, -1.0));
        assert_eq!(b.max(), &Point::new(1.0, 1.0, 1.0));
    }
}
