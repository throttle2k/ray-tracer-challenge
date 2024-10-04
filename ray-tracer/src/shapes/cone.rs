use approx_eq::{ApproxEq, EPSILON};

use crate::{
    rays::Ray,
    tuples::{points::Point, vectors::Vector, Tuple},
};

#[derive(Debug, Clone, PartialEq)]
pub enum ConeCap {
    Uncapped,
    TopCap,
    BottomCap,
    Both,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cone {
    minimum: f64,
    maximum: f64,
    cap: ConeCap,
}

impl Default for Cone {
    fn default() -> Self {
        Self {
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
            cap: ConeCap::Uncapped,
        }
    }
}

impl Cone {
    pub fn with_minimum(mut self, minimum: f64) -> Self {
        self.minimum = minimum;
        self
    }

    pub fn with_maximum(mut self, maximum: f64) -> Self {
        self.maximum = maximum;
        self
    }

    pub fn with_cap(mut self, cap: ConeCap) -> Self {
        self.cap = cap;
        self
    }

    pub fn normal_at(&self, object_point: Point) -> Vector {
        let x2 = object_point.x().powi(2);
        let y2 = object_point.y().powi(2);
        let z2 = object_point.z().powi(2);

        let dist = x2 + z2;

        if (self.cap == ConeCap::Both || self.cap == ConeCap::TopCap)
            && dist <= y2
            && object_point.y() >= self.maximum - EPSILON
        {
            Vector::y_norm()
        } else if (self.cap == ConeCap::Both || self.cap == ConeCap::BottomCap)
            && dist <= y2
            && object_point.y() <= self.minimum + EPSILON
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

    pub fn intersects(&self, r: Ray) -> Vec<f64> {
        let a = r.direction.x().powi(2) - r.direction.y().powi(2) + r.direction.z().powi(2);
        let b = 2.0 * r.origin.x() * r.direction.x() - 2.0 * r.origin.y() * r.direction.y()
            + 2.0 * r.origin.z() * r.direction.z();
        let c = r.origin.x().powi(2) - r.origin.y().powi(2) + r.origin.z().powi(2);
        let mut xs = if a.approx_eq(0.0) {
            if b.approx_eq(0.0) {
                Vec::new()
            } else {
                let t = -c / (2.0 * b);
                vec![t]
            }
        } else {
            let discriminant = b.powi(2) - 4.0 * a * c;

            if discriminant < 0.0 {
                Vec::new()
            } else {
                let t0 = (-b - (discriminant.sqrt())) / (2.0 * a);
                let t1 = (-b + (discriminant.sqrt())) / (2.0 * a);
                let (t0, t1) = if t0 > t1 { (t1, t0) } else { (t0, t1) };
                let mut xs = Vec::new();
                let y0 = r.origin.y() + t0 * r.direction.y();
                if self.minimum < y0 && y0 < self.maximum {
                    xs.push(t0);
                }
                let y1 = r.origin.y() + t1 * r.direction.y();
                if self.minimum < y1 && y1 < self.maximum {
                    xs.push(t1);
                }
                xs
            }
        };
        self.intersects_caps(r, &mut xs);
        xs
    }

    fn check_cap(r: Ray, t: f64, radius: f64) -> bool {
        let x = r.origin.x() + t * r.direction.x();
        let z = r.origin.z() + t * r.direction.z();
        x.powi(2) + z.powi(2) <= radius.powi(2)
    }

    fn intersects_caps(&self, r: Ray, xs: &mut Vec<f64>) {
        if self.cap == ConeCap::Uncapped || r.direction.y().approx_eq(0.0) {
            return;
        }
        if self.cap == ConeCap::Both || self.cap == ConeCap::BottomCap {
            let t = (self.minimum - r.origin.y()) / r.direction.y();
            if Cone::check_cap(r, t, self.minimum) {
                xs.push(t);
            }
        }
        if self.cap == ConeCap::Both || self.cap == ConeCap::TopCap {
            let t = (self.maximum - r.origin.y()) / r.direction.y();
            if Cone::check_cap(r, t, self.maximum) {
                xs.push(t);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::{INFINITY, NEG_INFINITY};

    use super::*;
    use yare::parameterized;

    #[test]
    fn a_ray_misses_a_cone() {
        let cone = Cone::default();
        let r = Ray::new(Point::new(1.0, 0.0, 0.0), Vector::z_norm());
        let xs = cone.intersects(r);
        assert_eq!(xs.len(), 0);
    }

    #[parameterized(
        ray_across_origin = {Point::new(0.0, 0.0, -5.0), Vector::z_norm(), 5.0, 5.0},
        ray_oblique_1 = {Point::new(0.0, 0.0, -5.0), Vector::new(1.0, 1.0, 1.0), 8.66025, 8.66025},
        ray_oblique_2 = {Point::new(1.0, 1.0, -5.0), Vector::new(-0.5, -1.0, 1.0), 4.55006, 49.44994},
    )]
    fn a_ray_strikes_a_cone(origin: Point, direction: Vector, t0: f64, t1: f64) {
        let cone = Cone::default();
        let direction = direction.normalize();
        let r = Ray::new(origin, direction);
        let xs = cone.intersects(r);
        assert_eq!(xs.len(), 2);
        assert!(xs[0].approx_eq(t0));
        assert!(xs[1].approx_eq(t1))
    }

    #[test]
    fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
        let cone = Cone::default();
        let direction = Vector::new(0.0, 1.0, 1.0).normalize();
        let r = Ray::new(Point::new(0.0, 0.0, -1.0), direction);
        let xs = cone.intersects(r);
        assert_eq!(xs.len(), 1);
        assert!(xs[0].approx_eq(0.35355));
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
        assert_eq!(cone.minimum, NEG_INFINITY);
        assert_eq!(cone.maximum, INFINITY);
    }

    #[parameterized(
        ray_outside_cone = {Point::new(0.0, 0.0, -5.0), Vector::y_norm(), 0},
        ray_oblique = {Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 1.0), 1},
        ray_vertical_close_to_origin = {Point::new(0.0, 0.0, -0.25), Vector::y_norm(), 2},
    )]
    fn intersecting_a_constrained_cone(point: Point, direction: Vector, count: usize) {
        let cone = Cone::default().with_minimum(-0.5).with_maximum(0.5);
        let direction = direction.normalize();
        let r = Ray::new(point, direction);
        let xs = cone.intersects(r);
        assert_eq!(xs.len(), count);
    }

    #[parameterized(
        ray_outside_cone = {Point::new(0.0, 0.0, -5.0), Vector::y_norm(), 0},
        ray_oblique = {Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 1.0), 2},
        ray_vertical_close_to_origin = {Point::new(0.0, 0.0, -0.25), Vector::y_norm(), 4},
    )]
    fn intersecting_the_caps_of_a_closed_cone(point: Point, direction: Vector, count: usize) {
        let cone = Cone::default()
            .with_minimum(-0.5)
            .with_maximum(0.5)
            .with_cap(ConeCap::Both);
        let direction = direction.normalize();
        let r = Ray::new(point, direction);
        let xs = cone.intersects(r);
        assert_eq!(xs.len(), count);
    }

    #[parameterized(
        ray_outside_cone = {Point::new(0.0, 0.0, -5.0), Vector::y_norm(), 0},
        ray_oblique = {Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 1.0), 1},
        ray_vertical_close_to_origin = {Point::new(0.0, 0.0, -0.25), Vector::y_norm(), 3},
    )]
    fn intersecting_the_cap_of_a_bottom_closed_cone(point: Point, direction: Vector, count: usize) {
        let cone = Cone::default()
            .with_minimum(-0.5)
            .with_maximum(0.5)
            .with_cap(ConeCap::BottomCap);
        let direction = direction.normalize();
        let r = Ray::new(point, direction);
        let xs = cone.intersects(r);
        assert_eq!(xs.len(), count);
    }

    #[parameterized(
        ray_outside_cone = {Point::new(0.0, 0.0, -5.0), Vector::y_norm(), 0},
        ray_oblique = {Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 1.0), 2},
        ray_vertical_close_to_origin = {Point::new(0.0, 0.0, -0.25), Vector::y_norm(), 3},
    )]
    fn intersecting_the_cap_of_a_top_closed_cone(point: Point, direction: Vector, count: usize) {
        let cone = Cone::default()
            .with_minimum(-0.5)
            .with_maximum(0.5)
            .with_cap(ConeCap::TopCap);
        let direction = direction.normalize();
        let r = Ray::new(point, direction);
        let xs = cone.intersects(r);
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
        let cone = Cone::default()
            .with_minimum(-0.5)
            .with_maximum(0.5)
            .with_cap(ConeCap::Both);
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
        let cone = Cone::default()
            .with_minimum(-0.5)
            .with_maximum(0.5)
            .with_cap(ConeCap::TopCap);
        let n = cone.normal_at(point);
        assert_eq!(n, normal);
    }
}
