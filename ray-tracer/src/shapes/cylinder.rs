use core::f64;

use approx_eq::{ApproxEq, EPSILON};

use crate::{
    bounds::Bounds,
    intersections::{Intersection, Intersections},
    rays::Ray,
    tuples::{points::Point, vectors::Vector, Tuple},
};

#[derive(Debug, Clone, PartialEq)]
pub enum CylinderCap {
    Uncapped,
    TopCap,
    BottomCap,
    Both,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cylinder {
    minimum: f64,
    maximum: f64,
    cap: CylinderCap,
}

impl Default for Cylinder {
    fn default() -> Self {
        Self {
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
            cap: CylinderCap::Uncapped,
        }
    }
}

impl Cylinder {
    pub fn with_minimum(mut self, minimum: f64) -> Self {
        self.minimum = minimum;
        self
    }

    pub fn with_maximum(mut self, maximum: f64) -> Self {
        self.maximum = maximum;
        self
    }

    pub fn with_cap(mut self, cap: CylinderCap) -> Self {
        self.cap = cap;
        self
    }

    pub fn normal_at(&self, object_point: Point) -> Vector {
        let dist = object_point.x().powi(2) + object_point.z().powi(2);

        if (self.cap == CylinderCap::Both || self.cap == CylinderCap::TopCap)
            && dist < 1.0
            && object_point.y() >= self.maximum - EPSILON
        {
            Vector::y_norm()
        } else if (self.cap == CylinderCap::Both || self.cap == CylinderCap::BottomCap)
            && dist < 1.0
            && object_point.y() <= self.minimum + EPSILON
        {
            Vector::y_norm() * -1.0
        } else {
            Vector::new(object_point.x(), 0.0, object_point.z())
        }
    }

    pub fn intersects(&self, object_id: usize, r: Ray) -> Intersections {
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
                if self.minimum < y0 && y0 < self.maximum {
                    xs.push(Intersection::new(t0, object_id));
                }
                let y1 = r.origin.y() + t1 * r.direction.y();
                if self.minimum < y1 && y1 < self.maximum {
                    xs.push(Intersection::new(t1, object_id));
                }
                xs
            }
        };
        self.intersects_caps(object_id, r, &mut intersections);
        intersections
    }

    fn check_cap(r: Ray, t: f64) -> bool {
        let x = r.origin.x() + t * r.direction.x();
        let z = r.origin.z() + t * r.direction.z();
        x.powi(2) + z.powi(2) <= 1.0
    }

    fn intersects_caps(&self, object_id: usize, r: Ray, xs: &mut Intersections) {
        if self.cap == CylinderCap::Uncapped {
            return;
        }
        if self.cap == CylinderCap::Both || self.cap == CylinderCap::BottomCap {
            let t = (self.minimum - r.origin.y()) / r.direction.y();
            if Cylinder::check_cap(r, t) {
                xs.push(Intersection::new(t, object_id));
            }
        }
        if self.cap == CylinderCap::Both || self.cap == CylinderCap::TopCap {
            let t = (self.maximum - r.origin.y()) / r.direction.y();
            if Cylinder::check_cap(r, t) {
                xs.push(Intersection::new(t, object_id));
            }
        }
    }

    pub fn bounds(&self) -> Bounds {
        Bounds::new(
            Point::new(-1.0, self.minimum, -1.0),
            Point::new(1.0, self.maximum, 1.0),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::f64::{INFINITY, NEG_INFINITY};

    use crate::{shapes::ObjectBuilder, REGISTRY};

    use super::*;
    use yare::parameterized;

    #[parameterized(
        ray_on_surface_vertical = {Point::new(1.0, 0.0, 0.0), Vector::y_norm()},
        ray_inside_vertical = {Point::zero(), Vector::y_norm()},
        ray_outside_oblique = {Point::new(0.0, 0.0, -5.0), Vector::new(1.0, 1.0, 1.0)},
    )]
    fn a_ray_misses_a_cylinder(origin: Point, direction: Vector) {
        let cyl = Cylinder::default();
        let cyl = ObjectBuilder::new_cylinder(cyl).register();
        let registry = REGISTRY.read().unwrap();
        let cyl = registry.get_object(cyl).unwrap();
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
        let cyl = Cylinder::default();
        let cyl = ObjectBuilder::new_cylinder(cyl).register();
        let registry = REGISTRY.read().unwrap();
        let cyl = registry.get_object(cyl).unwrap();
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
        assert_eq!(cyl.minimum, NEG_INFINITY);
        assert_eq!(cyl.maximum, INFINITY);
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
        let cyl = Cylinder::default().with_minimum(1.0).with_maximum(2.0);
        let cyl = ObjectBuilder::new_cylinder(cyl).register();
        let registry = REGISTRY.read().unwrap();
        let cyl = registry.get_object(cyl).unwrap();
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
        let cyl = Cylinder::default()
            .with_minimum(1.0)
            .with_maximum(2.0)
            .with_cap(CylinderCap::Both);
        let cyl = ObjectBuilder::new_cylinder(cyl).register();
        let registry = REGISTRY.read().unwrap();
        let cyl = registry.get_object(cyl).unwrap();
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
        let cyl = Cylinder::default()
            .with_minimum(1.0)
            .with_maximum(2.0)
            .with_cap(CylinderCap::BottomCap);
        let cyl = ObjectBuilder::new_cylinder(cyl).register();
        let registry = REGISTRY.read().unwrap();
        let cyl = registry.get_object(cyl).unwrap();
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
        let cyl = Cylinder::default()
            .with_minimum(1.0)
            .with_maximum(2.0)
            .with_cap(CylinderCap::TopCap);
        let cyl = ObjectBuilder::new_cylinder(cyl).register();
        let registry = REGISTRY.read().unwrap();
        let cyl = registry.get_object(cyl).unwrap();
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
        let cyl = Cylinder::default()
            .with_minimum(1.0)
            .with_maximum(2.0)
            .with_cap(CylinderCap::Both);
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
        let cyl = Cylinder::default()
            .with_minimum(1.0)
            .with_maximum(2.0)
            .with_cap(CylinderCap::TopCap);
        let n = cyl.normal_at(point);
        assert_eq!(n, normal);
    }
}
