use approx_eq::{ApproxEq, EPSILON};

use crate::{
    bounds::Bounds,
    intersections::{Intersection, Intersections},
    rays::Ray,
    tuples::{points::Point, vectors::Vector, Tuple},
};

use super::{Cap, Object};

#[derive(Debug, Clone, PartialEq)]
pub struct Cone {
    min: f64,
    max: f64,
    cap: Cap,
}

impl Default for Cone {
    fn default() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
            cap: Cap::Uncapped,
        }
    }
}

impl Cone {
    pub fn with_min(&mut self, min: f64) {
        self.min = min;
    }

    pub fn with_max(&mut self, max: f64) {
        self.max = max;
    }

    pub fn with_cap(&mut self, cap: Cap) {
        self.cap = cap;
    }

    fn intersects_caps<'a>(&self, object: &'a Object, r: &Ray, xs: &mut Intersections<'a>) {
        if self.cap == Cap::Uncapped || r.direction.y().approx_eq(0.0) {
            return;
        }
        if self.cap == Cap::Both || self.cap == Cap::BottomCap {
            let t = (self.min - r.origin.y()) / r.direction.y();
            if Cone::check_cap(r, t, self.min) {
                xs.push(Intersection::new(t, object));
            }
        }
        if self.cap == Cap::Both || self.cap == Cap::TopCap {
            let t = (self.max - r.origin.y()) / r.direction.y();
            if Cone::check_cap(r, t, self.max) {
                xs.push(Intersection::new(t, object));
            }
        }
    }

    fn check_cap(r: &Ray, t: f64, radius: f64) -> bool {
        let x = r.origin.x() + t * r.direction.x();
        let z = r.origin.z() + t * r.direction.z();
        x.powi(2) + z.powi(2) <= radius.powi(2)
    }

    pub fn normal_at(&self, object_point: Point) -> Vector {
        let x2 = object_point.x().powi(2);
        let y2 = object_point.y().powi(2);
        let z2 = object_point.z().powi(2);

        let dist = x2 + z2;

        if (self.cap == Cap::Both || self.cap == Cap::TopCap)
            && dist <= y2
            && object_point.y() >= self.max - EPSILON
        {
            Vector::y_norm()
        } else if (self.cap == Cap::Both || self.cap == Cap::BottomCap)
            && dist <= y2
            && object_point.y() <= self.min + EPSILON
        {
            Vector::y_norm() * -1.0
        } else {
            let y = if object_point.y() > 0.0 {
                -f64::sqrt(dist)
            } else {
                f64::sqrt(dist)
            };

            Vector::new(object_point.x(), y, object_point.z())
        }
    }

    pub fn intersects<'a>(&self, object: &'a Object, r: &Ray) -> Intersections<'a> {
        let a = r.direction.x().powi(2) - r.direction.y().powi(2) + r.direction.z().powi(2);
        let b = 2.0 * r.origin.x() * r.direction.x() - 2.0 * r.origin.y() * r.direction.y()
            + 2.0 * r.origin.z() * r.direction.z();
        let c = r.origin.x().powi(2) - r.origin.y().powi(2) + r.origin.z().powi(2);
        let mut intersections = if a.approx_eq(0.0) {
            if b.approx_eq(0.0) {
                Intersections::new()
            } else {
                let t = -c / (2.0 * b);
                let mut xs = Intersections::new();
                xs.push(Intersection::new(t, object));
                xs
            }
        } else {
            let discriminant = b.powi(2) - 4.0 * a * c;

            if discriminant < 0.0 {
                Intersections::new()
            } else {
                let t0 = (-b - (discriminant.sqrt())) / (2.0 * a);
                let t1 = (-b + (discriminant.sqrt())) / (2.0 * a);
                let (t0, t1) = if t0 > t1 { (t1, t0) } else { (t0, t1) };
                let mut xs = Intersections::new();
                let y0 = r.origin.y() + t0 * r.direction.y();
                if self.min < y0 && y0 < self.max {
                    xs.push(Intersection::new(t0, object));
                }
                let y1 = r.origin.y() + t1 * r.direction.y();
                if self.min < y1 && y1 < self.max {
                    xs.push(Intersection::new(t1, object));
                }
                xs
            }
        };
        self.intersects_caps(object, r, &mut intersections);
        intersections
    }

    pub fn bounds(&self) -> Bounds {
        let abs_min = self.min.abs();
        let abs_max = self.max.abs();
        let limit = abs_min.max(abs_max);
        Bounds::new(
            Point::new(-limit, self.min, -limit),
            Point::new(limit, self.max, limit),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::f64::{INFINITY, NEG_INFINITY};

    use crate::shapes::ObjectBuilder;

    use super::*;
    use yare::parameterized;

    #[test]
    fn a_ray_misses_a_cone() {
        let cone = ObjectBuilder::new_cone().build();
        let r = Ray::new(Point::new(1.0, 0.0, 0.0), Vector::z_norm());
        let xs = cone.intersects(&r);
        assert_eq!(xs.len(), 0);
    }

    #[parameterized(
        ray_across_origin = {Point::new(0.0, 0.0, -5.0), Vector::z_norm(), 5.0, 5.0},
        ray_oblique_1 = {Point::new(0.0, 0.0, -5.0), Vector::new(1.0, 1.0, 1.0), 8.66025, 8.66025},
        ray_oblique_2 = {Point::new(1.0, 1.0, -5.0), Vector::new(-0.5, -1.0, 1.0), 4.55006, 49.44994},
    )]
    fn a_ray_strikes_a_cone(origin: Point, direction: Vector, t0: f64, t1: f64) {
        let cone = ObjectBuilder::new_cone().build();
        let direction = direction.normalize();
        let r = Ray::new(origin, direction);
        let xs = cone.intersects(&r);
        assert_eq!(xs.len(), 2);
        assert!(xs[0].t.approx_eq(t0));
        assert!(xs[1].t.approx_eq(t1))
    }

    #[test]
    fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
        let cone = ObjectBuilder::new_cone().build();
        let direction = Vector::new(0.0, 1.0, 1.0).normalize();
        let r = Ray::new(Point::new(0.0, 0.0, -1.0), direction);
        let xs = cone.intersects(&r);
        assert_eq!(xs.len(), 1);
        assert!(xs[0].t.approx_eq(0.35355));
    }

    #[parameterized(
        positive_x = {Point::new(0.0, 0.0, 0.0), Vector::zero()},
        negative_z = {Point::new(1.0, 1.0, 1.0), Vector::new(1.0, -f64::sqrt(2.0), 1.0)},
        negative_x = {Point::new(-1.0, -1.0, 0.0), Vector::new(-1.0, 1.0, 0.0)},

    )]
    fn normal_vector_on_a_cone(point: Point, normal: Vector) {
        let cone = Cone::default();
        let n = cone.normal_at(point);
        assert_eq!(n, normal);
    }

    #[test]
    fn default_minimum_and_maximum_for_a_cone() {
        let cone = Cone::default();
        assert_eq!(cone.min, NEG_INFINITY);
        assert_eq!(cone.max, INFINITY);
    }

    #[parameterized(
        ray_outside_cone = {Point::new(0.0, 0.0, -5.0), Vector::y_norm(), 0},
        ray_oblique = {Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 1.0), 1},
        ray_vertical_close_to_origin = {Point::new(0.0, 0.0, -0.25), Vector::y_norm(), 2},
    )]
    fn intersecting_a_constrained_cone(point: Point, direction: Vector, count: usize) {
        let cone = ObjectBuilder::new_cone()
            .with_min(-0.5)
            .with_max(0.5)
            .build();
        let direction = direction.normalize();
        let r = Ray::new(point, direction);
        let xs = cone.intersects(&r);
        assert_eq!(xs.len(), count);
    }

    #[parameterized(
        ray_outside_cone = {Point::new(0.0, 0.0, -5.0), Vector::y_norm(), 0},
        ray_oblique = {Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 1.0), 2},
        ray_vertical_close_to_origin = {Point::new(0.0, 0.0, -0.25), Vector::y_norm(), 4},
    )]
    fn intersecting_the_caps_of_a_closed_cone(point: Point, direction: Vector, count: usize) {
        let cone = ObjectBuilder::new_cone()
            .with_min(-0.5)
            .with_max(0.5)
            .with_cap(Cap::Both)
            .build();
        let direction = direction.normalize();
        let r = Ray::new(point, direction);
        let xs = cone.intersects(&r);
        assert_eq!(xs.len(), count);
    }

    #[parameterized(
        ray_outside_cone = {Point::new(0.0, 0.0, -5.0), Vector::y_norm(), 0},
        ray_oblique = {Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 1.0), 1},
        ray_vertical_close_to_origin = {Point::new(0.0, 0.0, -0.25), Vector::y_norm(), 3},
    )]
    fn intersecting_the_cap_of_a_bottom_closed_cone(point: Point, direction: Vector, count: usize) {
        let cone = ObjectBuilder::new_cone()
            .with_min(-0.5)
            .with_max(0.5)
            .with_cap(Cap::BottomCap)
            .build();
        let direction = direction.normalize();
        let r = Ray::new(point, direction);
        let xs = cone.intersects(&r);
        assert_eq!(xs.len(), count);
    }

    #[parameterized(
        ray_outside_cone = {Point::new(0.0, 0.0, -5.0), Vector::y_norm(), 0},
        ray_oblique = {Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 1.0), 2},
        ray_vertical_close_to_origin = {Point::new(0.0, 0.0, -0.25), Vector::y_norm(), 3},
    )]
    fn intersecting_the_cap_of_a_top_closed_cone(point: Point, direction: Vector, count: usize) {
        let cone = ObjectBuilder::new_cone()
            .with_min(-0.5)
            .with_max(0.5)
            .with_cap(Cap::TopCap)
            .build();
        let direction = direction.normalize();
        let r = Ray::new(point, direction);
        let xs = cone.intersects(&r);
        assert_eq!(xs.len(), count);
    }

    #[parameterized(
        normal_on_bottom_cap_1 = {Point::new(0.0, -0.5, 0.0), Vector::y_norm() * -1.0},
        normal_on_bottom_cap_2 = {Point::new(0.5, -0.5, 0.0), Vector::y_norm() * -1.0},
        normal_on_bottom_cap_3 = {Point::new(0.0, -0.5, 0.5), Vector::y_norm() * -1.0},
        normal_on_top_cap_1 = {Point::new(0.0, 0.5, 0.0), Vector::y_norm()},
        normal_on_top_cap_2 = {Point::new(0.5, 0.5, 0.0), Vector::y_norm()},
        normal_on_top_cap_3 = {Point::new(0.0, 0.5, 0.5), Vector::y_norm()},
    )]
    fn the_normal_vector_on_a_cone_end_cap(point: Point, normal: Vector) {
        let mut cone = Cone::default();
        cone.with_min(-0.5);
        cone.with_max(0.5);
        cone.with_cap(Cap::Both);
        let n = cone.normal_at(point);
        assert_eq!(n, normal);
    }

    #[parameterized(
        normal_on_bottom_1 = {Point::new(0.0, -0.5, 0.0), Vector::zero()},
        normal_on_bottom_2 = {Point::new(0.5, -0.5, 0.0), Vector::new(0.5, 0.5, 0.0)},
        normal_on_bottom_3 = {Point::new(0.0, -0.5, 0.5), Vector::new(0.0, 0.5, 0.5)},
        normal_on_top_cap_1 = {Point::new(0.0, 0.5, 0.0), Vector::y_norm()},
        normal_on_top_cap_2 = {Point::new(0.5, 0.5, 0.0), Vector::y_norm()},
        normal_on_top_cap_3 = {Point::new(0.0, 0.5, 0.5), Vector::y_norm()},
    )]
    fn the_normal_vector_on_a_cone_with_top_cap(point: Point, normal: Vector) {
        let mut cone = Cone::default();
        cone.with_min(-0.5);
        cone.with_max(0.5);
        cone.with_cap(Cap::TopCap);
        let n = cone.normal_at(point);
        assert_eq!(n, normal);
    }

    #[test]
    fn an_unbounded_cone_has_a_bounding_box() {
        let s = Cone::default();
        let b = s.bounds();
        assert!(b.min().x().is_infinite());
        assert!(b.min().x().is_sign_negative());
        assert!(b.min().y().is_infinite());
        assert!(b.min().y().is_sign_negative());
        assert!(b.min().z().is_infinite());
        assert!(b.min().z().is_sign_negative());
        assert!(b.max().x().is_infinite());
        assert!(b.max().x().is_sign_positive());
        assert!(b.max().y().is_infinite());
        assert!(b.max().y().is_sign_positive());
        assert!(b.max().z().is_infinite());
        assert!(b.max().z().is_sign_positive());
    }

    #[test]
    fn a_bounded_cone_has_a_bounding_box() {
        let mut s = Cone::default();
        s.with_min(-5.0);
        s.with_max(3.0);
        let b = s.bounds();
        assert_eq!(b.min(), &Point::new(-5.0, -5.0, -5.0));
        assert_eq!(b.max(), &Point::new(5.0, 3.0, 5.0));
    }
}
