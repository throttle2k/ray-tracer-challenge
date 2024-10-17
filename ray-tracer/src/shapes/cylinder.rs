use core::f64;

use approx_eq::{ApproxEq, EPSILON};

use crate::{
    bounds::Bounds,
    intersections::{Intersection, Intersections},
    rays::Ray,
    tuples::{points::Point, vectors::Vector, Tuple},
};

use super::{Cap, Object};

#[derive(Debug, Clone, PartialEq)]
pub struct Cylinder {
    min: f64,
    max: f64,
    cap: Cap,
}

impl Default for Cylinder {
    fn default() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
            cap: Cap::Uncapped,
        }
    }
}

impl Cylinder {
    pub fn with_min(&mut self, min: f64) {
        self.min = min;
    }

    pub fn with_max(&mut self, max: f64) {
        self.max = max;
    }

    pub fn with_cap(&mut self, cap: Cap) {
        self.cap = cap;
    }

    fn check_cap(r: &Ray, t: f64) -> bool {
        let x = r.origin.x() + t * r.direction.x();
        let z = r.origin.z() + t * r.direction.z();
        x.powi(2) + z.powi(2) <= 1.0
    }

    fn intersects_caps<'a>(&self, object: &'a Object, r: &Ray, xs: &mut Intersections<'a>) {
        if self.cap == Cap::Uncapped {
            return;
        }
        if self.cap == Cap::Both || self.cap == Cap::BottomCap {
            let t = (self.min - r.origin.y()) / r.direction.y();
            if Cylinder::check_cap(r, t) {
                xs.push(Intersection::new(t, object));
            }
        }
        if self.cap == Cap::Both || self.cap == Cap::TopCap {
            let t = (self.max - r.origin.y()) / r.direction.y();
            if Cylinder::check_cap(r, t) {
                xs.push(Intersection::new(t, object));
            }
        }
    }

    pub fn normal_at(&self, object_point: Point) -> Vector {
        let dist = object_point.x().powi(2) + object_point.z().powi(2);

        if (self.cap == Cap::Both || self.cap == Cap::TopCap)
            && dist < 1.0
            && object_point.y() >= self.max - EPSILON
        {
            Vector::y_norm()
        } else if (self.cap == Cap::Both || self.cap == Cap::BottomCap)
            && dist < 1.0
            && object_point.y() <= self.min + EPSILON
        {
            Vector::y_norm() * -1.0
        } else {
            Vector::new(object_point.x(), 0.0, object_point.z())
        }
    }

    pub fn intersects<'a>(&self, object: &'a Object, r: &Ray) -> Intersections<'a> {
        let a = r.direction.x().powi(2) + r.direction.z().powi(2);
        let mut intersections = if a.approx_eq(0.0) {
            Intersections::new()
        } else {
            let b = 2.0 * r.origin.x() * r.direction.x() + 2.0 * r.origin.z() * r.direction.z();
            let c = r.origin.x().powi(2) + r.origin.z().powi(2) - 1.0;
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
        Bounds::new(
            Point::new(-1.0, self.min, -1.0),
            Point::new(1.0, self.max, 1.0),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::f64::{INFINITY, NEG_INFINITY};

    use crate::shapes::ObjectBuilder;

    use super::*;
    use yare::parameterized;

    #[parameterized(
        ray_on_surface_vertical = {Point::new(1.0, 0.0, 0.0), Vector::y_norm()},
        ray_inside_vertical = {Point::zero(), Vector::y_norm()},
        ray_outside_oblique = {Point::new(0.0, 0.0, -5.0), Vector::new(1.0, 1.0, 1.0)},
    )]
    fn a_ray_misses_a_cylinder(origin: Point, direction: Vector) {
        let cyl = ObjectBuilder::new_cylinder().build();
        let direction = direction.normalize();
        let r = Ray::new(origin, direction);
        let xs = cyl.intersects(&r);
        assert_eq!(xs.len(), 0);
    }

    #[parameterized(
        ray_tangent = {Point::new(1.0, 0.0, -5.0), Vector::z_norm(), 5.0, 5.0},
        ray_perpendicular = {Point::new(0.0, 0.0, -5.0), Vector::z_norm(), 4.0, 6.0},
        ray_oblique = {Point::new(0.5, 0.0, -5.0), Vector::new(0.1, 1.0, 1.0), 6.80798, 7.08872},
    )]
    fn a_ray_strikes_a_cylinder(origin: Point, direction: Vector, t0: f64, t1: f64) {
        let cyl = ObjectBuilder::new_cylinder().build();
        let direction = direction.normalize();
        let r = Ray::new(origin, direction);
        let xs = cyl.intersects(&r);
        assert_eq!(xs.len(), 2);
        assert!(xs[0].t.approx_eq(t0));
        assert!(xs[1].t.approx_eq(t1))
    }

    #[parameterized(
        positive_x = {Point::new(1.0, 0.0, 0.0), Vector::x_norm()},
        negative_z = {Point::new(0.0, 5.0, -1.0), Vector::new(0.0, 0.0, -1.0)},
        positive_z = {Point::new(0.0, -2.0, 1.0), Vector::z_norm()},
        negative_x = {Point::new(-1.0, 1.0, 0.0), Vector::new(-1.0, 0.0, 0.0)},

    )]
    fn normal_vector_on_a_cylinder(point: Point, normal: Vector) {
        let cyl = Cylinder::default();
        let n = cyl.normal_at(point);
        assert_eq!(n, normal);
    }

    #[test]
    fn default_minimum_and_maximum_for_a_cylinder() {
        let cyl = Cylinder::default();
        assert_eq!(cyl.min, NEG_INFINITY);
        assert_eq!(cyl.max, INFINITY);
    }

    #[parameterized(
        ray_oblique_inside_without_intersections = {Point::new(0.0, 1.5, 0.0), Vector::new(0.1, 1.0, 0.0), 0},
        ray_perpendicular_above = {Point::new(0.0, 3.0, -5.0), Vector::z_norm(), 0},
        ray_perpendicular_below = {Point::new(0.0, 0.0, -5.0), Vector::z_norm(), 0},
        ray_perpendicular_on_top_extreme = {Point::new(0.0, 2.0, -5.0), Vector::z_norm(), 0},
        ray_perpendicular_on_bottom_extreme = {Point::new(0.0, 1.0, -5.0), Vector::z_norm(), 0},
        ray_perpendicular_in_middle = {Point::new(0.0, 1.5, -2.0), Vector::z_norm(), 2},
    )]
    fn intersecting_a_constrained_cylinder(point: Point, direction: Vector, count: usize) {
        let cyl = ObjectBuilder::new_cylinder()
            .with_min(1.0)
            .with_max(2.0)
            .build();
        let direction = direction.normalize();
        let r = Ray::new(point, direction);
        let xs = cyl.intersects(&r);
        assert_eq!(xs.len(), count);
    }

    #[parameterized(
        ray_straight_across_two_caps = {Point::new(0.0, 3.0, 0.0), Vector::new(0.0, -1.0, 0.0)},
        ray_oblique_across_two_caps_from_above = {Point::new(0.0, 3.0, -2.0), Vector::new(0.0, -1.0, 2.0)},
        ray_oblique_across_two_caps_from_above_to_corner = {Point::new(0.0, 4.0, -2.0), Vector::new(0.0, -1.0, 1.0)},
        ray_oblique_across_two_caps_from_below = {Point::new(0.0, 0.0, -2.0), Vector::new(0.0, 1.0, 2.0)},
        ray_oblique_across_two_caps_from_below_to_corner = {Point::new(0.0, -1.0, -2.0), Vector::new(0.0, 1.0, 1.0)},
    )]
    fn intersecting_the_caps_of_a_closed_cylinder(point: Point, direction: Vector) {
        let cyl = ObjectBuilder::new_cylinder()
            .with_min(1.0)
            .with_max(2.0)
            .with_cap(Cap::Both)
            .build();
        let direction = direction.normalize();
        let r = Ray::new(point, direction);
        let xs = cyl.intersects(&r);
        assert_eq!(xs.len(), 2);
    }

    #[parameterized(
        ray_straight_across_cap = {Point::new(0.0, 3.0, 0.0), Vector::new(0.0, -1.0, 0.0), 1},
        ray_oblique_across_cap_from_above = {Point::new(0.0, 3.0, -2.0), Vector::new(0.0, -1.0, 2.0), 1},
        ray_oblique_across_cap_from_above_to_corner = {Point::new(0.0, 4.0, -2.0), Vector::new(0.0, -1.0, 1.0), 1},
        ray_oblique_across_cap_from_below = {Point::new(0.0, 0.0, -2.0), Vector::new(0.0, 1.0, 2.0), 2},
        ray_oblique_across_cap_from_below_to_corner = {Point::new(0.0, -1.0, -2.0), Vector::new(0.0, 1.0, 1.0), 1},
    )]
    fn intersecting_the_cap_of_a_bottom_closed_cylinder(
        point: Point,
        direction: Vector,
        count: usize,
    ) {
        let cyl = ObjectBuilder::new_cylinder()
            .with_min(1.0)
            .with_max(2.0)
            .with_cap(Cap::BottomCap)
            .build();
        let direction = direction.normalize();
        let r = Ray::new(point, direction);
        let xs = cyl.intersects(&r);
        assert_eq!(xs.len(), count);
    }

    #[parameterized(
        ray_straight_across_cap = {Point::new(0.0, 3.0, 0.0), Vector::new(0.0, -1.0, 0.0), 1},
        ray_oblique_across_cap_from_above = {Point::new(0.0, 3.0, -2.0), Vector::new(0.0, -1.0, 2.0), 2},
        ray_oblique_across_cap_from_above_to_corner = {Point::new(0.0, 4.0, -2.0), Vector::new(0.0, -1.0, 1.0), 1},
        ray_oblique_across_cap_from_below = {Point::new(0.0, 0.0, -2.0), Vector::new(0.0, 1.0, 2.0), 1},
        ray_oblique_across_cap_from_below_to_corner = {Point::new(0.0, -1.0, -2.0), Vector::new(0.0, 1.0, 1.0), 1},
    )]
    fn intersecting_the_cap_of_a_top_closed_cylinder(
        point: Point,
        direction: Vector,
        count: usize,
    ) {
        let cyl = ObjectBuilder::new_cylinder()
            .with_min(1.0)
            .with_max(2.0)
            .with_cap(Cap::TopCap)
            .build();
        let direction = direction.normalize();
        let r = Ray::new(point, direction);
        let xs = cyl.intersects(&r);
        assert_eq!(xs.len(), count);
    }

    #[parameterized(
        normal_on_bottom_cap_1 = {Point::new(0.0, 1.0, 0.0), Vector::y_norm() * -1.0},
        normal_on_bottom_cap_2 = {Point::new(0.5, 1.0, 0.0), Vector::y_norm() * -1.0},
        normal_on_bottom_cap_3 = {Point::new(0.0, 1.0, 0.5), Vector::y_norm() * -1.0},
        normal_on_top_cap_1 = {Point::new(0.0, 2.0, 0.0), Vector::y_norm()},
        normal_on_top_cap_2 = {Point::new(0.5, 2.0, 0.0), Vector::y_norm()},
        normal_on_top_cap_3 = {Point::new(0.0, 2.0, 0.5), Vector::y_norm()},
    )]
    fn the_normal_vector_on_a_cylinder_end_cap(point: Point, normal: Vector) {
        let mut cyl = Cylinder::default();
        cyl.with_min(1.0);
        cyl.with_max(2.0);
        cyl.with_cap(Cap::Both);
        let n = cyl.normal_at(point);
        assert_eq!(n, normal);
    }

    #[parameterized(
        normal_on_bottom_1 = {Point::new(0.0, 1.0, 0.0), Vector::zero()},
        normal_on_bottom_2 = {Point::new(0.5, 1.0, 0.0), Vector::new(0.5, 0.0, 0.0)},
        normal_on_bottom_3 = {Point::new(0.0, 1.0, 0.5), Vector::new(0.0, 0.0, 0.5)},
        normal_on_top_cap_1 = {Point::new(0.0, 2.0, 0.0), Vector::y_norm()},
        normal_on_top_cap_2 = {Point::new(0.5, 2.0, 0.0), Vector::y_norm()},
        normal_on_top_cap_3 = {Point::new(0.0, 2.0, 0.5), Vector::y_norm()},
    )]
    fn the_normal_vector_on_a_cylinder_with_top_cap(point: Point, normal: Vector) {
        let mut cyl = Cylinder::default();
        cyl.with_min(1.0);
        cyl.with_max(2.0);
        cyl.with_cap(Cap::TopCap);
        let n = cyl.normal_at(point);
        assert_eq!(n, normal);
    }

    #[test]
    fn an_unbounded_cylinder_has_a_bounding_box() {
        let s = Cylinder::default();
        let b = s.bounds();
        assert_eq!(b.min().x(), -1.0);
        assert!(b.min().y().is_infinite());
        assert!(b.min().y().is_sign_negative());
        assert_eq!(b.min().z(), -1.0);
        assert_eq!(b.max().x(), 1.0);
        assert!(b.max().y().is_infinite());
        assert!(b.max().y().is_sign_positive());
        assert_eq!(b.max().z(), 1.0);
    }

    #[test]
    fn a_bounded_cylinder_has_a_bounding_box() {
        let mut s = Cylinder::default();
        s.with_min(-5.0);
        s.with_max(3.0);
        let b = s.bounds();
        assert_eq!(b.min(), &Point::new(-1.0, -5.0, -1.0));
        assert_eq!(b.max(), &Point::new(1.0, 3.0, 1.0));
    }
}
