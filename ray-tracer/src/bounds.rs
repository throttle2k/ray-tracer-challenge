use std::ops::Add;

use crate::{
    rays::Ray,
    transformations::Transformation,
    tuples::{points::Point, Tuple},
};

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    min: Point,
    max: Point,
}

impl PartialEq for Bounds {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max
    }
}

impl Bounds {
    pub fn new(min: Point, max: Point) -> Self {
        Self { min, max }
    }

    pub fn min(&self) -> &Point {
        &self.min
    }

    pub fn max(&self) -> &Point {
        &self.max
    }

    fn check_axis(origin: f64, direction: f64, min: f64, max: f64) -> (f64, f64) {
        let tmin_numerator = min - origin;
        let tmax_numerator = max - origin;
        let tmin = tmin_numerator / direction;
        let tmax = tmax_numerator / direction;
        if tmin < tmax {
            (tmin, tmax)
        } else {
            (tmax, tmin)
        }
    }

    pub fn intersects(&self, ray: &Ray) -> bool {
        let (xtmin, xtmax) = Bounds::check_axis(
            ray.origin.x(),
            ray.direction.x(),
            self.min.x(),
            self.max.x(),
        );
        let (ytmin, ytmax) = Bounds::check_axis(
            ray.origin.y(),
            ray.direction.y(),
            self.min.y(),
            self.max.y(),
        );
        let (ztmin, ztmax) = Bounds::check_axis(
            ray.origin.z(),
            ray.direction.z(),
            self.min.z(),
            self.max.z(),
        );

        let tmax = xtmax.min(ytmax.min(ztmax));
        if tmax < 0.0 {
            false
        } else {
            let tmin = xtmin.max(ytmin.max(ztmin));

            tmin <= tmax
        }
    }

    pub fn transform(&self, transformation: &Transformation) -> Bounds {
        let p1 = &transformation.matrix * &self.min;
        let p2 = &transformation.matrix * &Point::new(self.min.x(), self.min.y(), self.max.z());
        let p3 = &transformation.matrix * &Point::new(self.min.x(), self.max.y(), self.min.z());
        let p4 = &transformation.matrix * &Point::new(self.min.x(), self.max.y(), self.max.z());
        let p5 = &transformation.matrix * &Point::new(self.max.x(), self.min.y(), self.min.z());
        let p6 = &transformation.matrix * &Point::new(self.max.x(), self.min.y(), self.max.z());
        let p7 = &transformation.matrix * &Point::new(self.max.x(), self.max.y(), self.min.z());
        let p8 = &transformation.matrix * &self.max;

        Bounds::default() + &p1 + &p2 + &p3 + &p4 + &p5 + &p6 + &p7 + &p8
    }

    pub fn contains(&self, other: &Bounds) -> bool {
        self.contains_point(other.min()) && self.contains_point(other.max())
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        self.min().x() <= point.x()
            && self.min().y() <= point.y()
            && self.min().z() <= point.z()
            && self.max().x() >= point.x()
            && self.max().y() >= point.y()
            && self.max().z() >= point.z()
    }

    pub fn split(&self) -> (Bounds, Bounds) {
        let x_size = self.max.x() - self.min().x();
        let y_size = self.max.y() - self.min().y();
        let z_size = self.max.z() - self.min().z();
        let max_size = x_size.max(y_size.max(z_size));
        let (mut x0, mut y0, mut z0) = (self.min.x(), self.min.y(), self.min.z());
        let (mut x1, mut y1, mut z1) = (self.max.x(), self.max.y(), self.max.z());
        if max_size == x_size {
            x1 = x0 + x_size / 2.0;
            x0 = x1;
        } else if max_size == y_size {
            y1 = y0 + y_size / 2.0;
            y0 = y1;
        } else {
            z1 = z0 + z_size / 2.0;
            z0 = z1;
        };
        let mid_min = Point::new(x0, y0, z0);
        let mid_max = Point::new(x1, y1, z1);
        let left = Bounds::new(self.min, mid_max);
        let right = Bounds::new(mid_min, self.max);
        (left, right)
    }
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            min: Point::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            max: Point::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
        }
    }
}

impl Add for Bounds {
    type Output = Bounds;

    fn add(self, rhs: Self) -> Self::Output {
        self + &rhs.min + &rhs.max
    }
}

impl Add<&Point> for Bounds {
    type Output = Bounds;

    fn add(mut self, rhs: &Point) -> Self::Output {
        let min_x = self.min.x().min(rhs.x());
        let min_y = self.min.y().min(rhs.y());
        let min_z = self.min.z().min(rhs.z());
        let max_x = self.max.x().max(rhs.x());
        let max_y = self.max.y().max(rhs.y());
        let max_z = self.max.z().max(rhs.z());

        self.min = Point::new(min_x, min_y, min_z);
        self.max = Point::new(max_x, max_y, max_z);
        self
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use approx_eq::ApproxEq;
    use yare::parameterized;

    use crate::tuples::vectors::Vector;

    use super::*;

    #[test]
    fn creating_an_empty_bounding_box() {
        let b = Bounds::default();
        assert!(b.min.x().is_infinite() && b.min.x().is_sign_positive());
        assert!(b.min.y().is_infinite() && b.min.y().is_sign_positive());
        assert!(b.min.z().is_infinite() && b.min.z().is_sign_positive());
        assert!(b.max.x().is_infinite() && b.max.x().is_sign_negative());
        assert!(b.max.y().is_infinite() && b.max.y().is_sign_negative());
        assert!(b.max.z().is_infinite() && b.max.z().is_sign_negative());
    }

    #[test]
    fn creating_a_bounding_box_with_volume() {
        let b = Bounds::new(Point::new(-1.0, -2.0, -3.0), Point::new(3.0, 2.0, 1.0));
        assert_eq!(b.min, Point::new(-1.0, -2.0, -3.0));
        assert_eq!(b.max, Point::new(3.0, 2.0, 1.0));
    }

    #[test]
    fn adding_points_to_empty_bounding_box() {
        let b = Bounds::default();
        let p1 = Point::new(-5.0, 2.0, 0.0);
        let p2 = Point::new(7.0, 0.0, -3.0);
        let b = b + &p1;
        let b = b + &p2;
        assert_eq!(b.min, Point::new(-5.0, 0.0, -3.0));
        assert_eq!(b.max, Point::new(7.0, 2.0, 0.0));
    }

    #[test]
    fn adding_a_bounding_box_to_another() {
        let mut box_1 = Bounds::new(Point::new(-5.0, -2.0, 0.0), Point::new(7.0, 4.0, 4.0));
        let box_2 = Bounds::new(Point::new(8.0, -7.0, -2.0), Point::new(14.0, 2.0, 8.0));
        box_1 = box_1 + box_2;
        assert_eq!(box_1.min, Point::new(-5.0, -7.0, -2.0));
        assert_eq!(box_1.max, Point::new(14.0, 4.0, 8.0));
    }

    #[parameterized(
        point_inside_1 = {Point::new(5.0, -2.0, 0.0), true},
        point_inside_2 = {Point::new(11.0, 4.0, 7.0), true},
        point_inside_3 = {Point::new(8.0, 1.0, 3.0), true},
        point_outside_1 = {Point::new(3.0, 0.0, 3.0), false},
        point_outside_2 = {Point::new(8.0, -4.0, 3.0), false},
        point_outside_3 = {Point::new(8.0, 1.0, -1.0), false},
        point_outside_4 = {Point::new(13.0, 1.0, 3.0), false},
        point_outside_5 = {Point::new(8.0, 5.0, 3.0), false},
        point_outside_6 = {Point::new(9.0, 1.0, 8.0), false},
    )]
    fn checking_if_a_box_contains_a_given_point(point: Point, result: bool) {
        let b = Bounds::new(Point::new(5.0, -2.0, 0.0), Point::new(11.0, 4.0, 7.0));
        assert_eq!(b.contains_point(&point), result);
    }

    #[parameterized(
        box_inside_1 = {Point::new(5.0, -2.0, 0.0), Point::new(11.0, 4.0, 7.0), true},
        box_inside_2 = {Point::new(6.0, -1.0, 1.0), Point::new(10.0, 3.0, 6.0), true},
        box_outside_1 = {Point::new(4.0,-3.0,-1.0), Point::new(10.0, 3.0,6.0), false},
        box_outside_2 = {Point::new(6.0, -1.0, 1.0), Point::new(12.0, 5.0, 8.0),false}
    )]
    fn checking_id_a_box_contains_a_given_box(min: Point, max: Point, result: bool) {
        let box_1 = Bounds::new(Point::new(5.0, -2.0, 0.0), Point::new(11.0, 4.0, 7.0));
        let box_2 = Bounds::new(min, max);
        assert_eq!(box_1.contains(&box_2), result);
    }

    #[test]
    fn transforming_a_bounding_box() {
        let b = Bounds::new(Point::new(-1.0, -1.0, -1.0), Point::new(1.0, 1.0, 1.0));
        let t = Transformation::new_transform()
            .rotation_y(PI / 4.0)
            .rotation_x(PI / 4.0);
        let b_2 = b.transform(&t);
        assert!(b_2.min.x().approx_eq(-1.41421));
        assert!(b_2.min.y().approx_eq(-1.707107));
        assert!(b_2.min.z().approx_eq(-1.707107));
        assert!(b_2.max.x().approx_eq(1.41421));
        assert!(b_2.max.y().approx_eq(1.707107));
        assert!(b_2.max.z().approx_eq(1.707107));
    }

    #[parameterized(
        intersecting_1 = {Point::new(5.0, 0.5, 0.0), Vector::new(-1.0, 0.0, 0.0), true},
        intersecting_2 = {Point::new(-5.0, 0.5, 0.0), Vector::new(1.0, 0.0, 0.0), true},
        intersecting_3 = {Point::new(0.5, 5.0, 0.0), Vector::new(0.0, -1.0, 0.0), true},
        intersecting_4 = {Point::new(0.5, -5.0, 0.0), Vector::new(0.0, 1.0, 0.0), true},
        intersecting_5 = {Point::new(0.5, 0.0, 5.0), Vector::new(0.0, 0.0, -1.0), true},
        intersecting_6 = {Point::new(0.5, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0), true},
        intersecting_7 = {Point::new(0.0, 0.5, 0.0), Vector::new(0.0, 0.0, 1.0), true},
        not_intersecting_1 = {Point::new(-2.0, 0.0, 0.0), Vector::new(2.0, 4.0, 6.0), false},
        not_intersecting_2 = {Point::new(0.0, -2.0, 0.0), Vector::new(6.0, 2.0, 4.0), false},
        not_intersecting_3 = {Point::new(0.0, 0.0, -2.0), Vector::new(4.0, 6.0, 2.0), false},
        not_intersecting_4 = {Point::new(2.0, 0.0, 2.0), Vector::new(0.0, 0.0, -1.0), false},
        not_intersecting_5 = {Point::new(0.0, 2.0, 2.0), Vector::new(0.0, -1.0, 0.0), false},
        not_intersecting_6 = {Point::new(2.0, 2.0, 0.0), Vector::new(-1.0, 0.0, 0.0), false},
    )]
    fn intersecting_a_ray_with_bounding_box_at_the_origin(
        origin: Point,
        direction: Vector,
        result: bool,
    ) {
        let b = Bounds::new(Point::new(-1.0, -1.0, -1.0), Point::new(1.0, 1.0, 1.0));
        let direction = direction.normalize();
        let r = Ray::new(origin, direction);
        assert_eq!(b.intersects(&r), result);
    }

    #[parameterized(
        intersecting_1 = {Point::new(15.0, 1.0, 2.0), Vector::new(-1.0, 0.0, 0.0), true},
        intersecting_2 = {Point::new(-5.0, 1.0, 4.0), Vector::new(1.0, 0.0, 0.0), true},
        intersecting_3 = {Point::new(7.0, 6.0, 5.0), Vector::new(0.0, -1.0, 0.0), true},
        intersecting_4 = {Point::new(9.0, -5.0, 6.0), Vector::new(0.0, 1.0, 0.0), true},
        intersecting_5 = {Point::new(8.0, 2.0, 12.0), Vector::new(0.0, 0.0, -1.0), true},
        intersecting_6 = {Point::new(6.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0), true},
        intersecting_7 = {Point::new(8.0, 1.0, 3.5), Vector::new(0.0, 0.0, 1.0), true},
        not_intersecting_1 = {Point::new(9.0, -1.0, -8.0), Vector::new(2.0, 4.0, 6.0), false},
        not_intersecting_2 = {Point::new(8.0, 3.0, -4.0), Vector::new(6.0, 2.0, 4.0), false},
        not_intersecting_3 = {Point::new(9.0, -1.0, -2.0), Vector::new(4.0, 6.0, 2.0), false},
        not_intersecting_4 = {Point::new(4.0, 0.0, 9.0), Vector::new(0.0, 0.0, -1.0), false},
        not_intersecting_5 = {Point::new(8.0, 6.0, -1.0), Vector::new(0.0, -1.0, 0.0), false},
        not_intersecting_6 = {Point::new(12.0, 5.0, 4.0), Vector::new(-1.0, 0.0, 0.0), false},
    )]
    fn intersecting_a_ray_with_a_non_cubic_bounding_box(
        origin: Point,
        direction: Vector,
        result: bool,
    ) {
        let b = Bounds::new(Point::new(5.0, -2.0, 0.0), Point::new(11.0, 4.0, 7.0));
        let direction = direction.normalize();
        let r = Ray::new(origin, direction);
        assert_eq!(b.intersects(&r), result);
    }

    #[test]
    fn splitting_a_perfect_cube() {
        let b = Bounds::new(Point::new(-1.0, -4.0, -5.0), Point::new(9.0, 6.0, 5.0));
        let (left, right) = b.split();
        assert_eq!(left.min, Point::new(-1.0, -4.0, -5.0));
        assert_eq!(left.max, Point::new(4.0, 6.0, 5.0));
        assert_eq!(right.min, Point::new(4.0, -4.0, -5.0));
        assert_eq!(right.max, Point::new(9.0, 6.0, 5.0));
    }

    #[test]
    fn splitting_a_x_wide_box() {
        let b = Bounds::new(Point::new(-1.0, -2.0, -3.0), Point::new(9.0, 5.5, 3.0));
        let (left, right) = b.split();
        assert_eq!(left.min, Point::new(-1.0, -2.0, -3.0));
        assert_eq!(left.max, Point::new(4.0, 5.5, 3.0));
        assert_eq!(right.min, Point::new(4.0, -2.0, -3.0));
        assert_eq!(right.max, Point::new(9.0, 5.5, 3.0));
    }

    #[test]
    fn splitting_a_y_wide_box() {
        let b = Bounds::new(Point::new(-1.0, -2.0, -3.0), Point::new(5.0, 8.0, 3.0));
        let (left, right) = b.split();
        assert_eq!(left.min, Point::new(-1.0, -2.0, -3.0));
        assert_eq!(left.max, Point::new(5.0, 3.0, 3.0));
        assert_eq!(right.min, Point::new(-1.0, 3.0, -3.0));
        assert_eq!(right.max, Point::new(5.0, 8.0, 3.0));
    }

    #[test]
    fn splitting_a_z_wide_box() {
        let b = Bounds::new(Point::new(-1.0, -2.0, -3.0), Point::new(5.0, 3.0, 7.0));
        let (left, right) = b.split();
        assert_eq!(left.min, Point::new(-1.0, -2.0, -3.0));
        assert_eq!(left.max, Point::new(5.0, 3.0, 2.0));
        assert_eq!(right.min, Point::new(-1.0, -2.0, 2.0));
        assert_eq!(right.max, Point::new(5.0, 3.0, 7.0));
    }
}
