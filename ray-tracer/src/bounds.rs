use std::ops::Add;

use crate::{
    rays::Ray,
    transformations::Transformation,
    tuples::{points::Point, Tuple},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Bounds {
    min: Point,
    max: Point,
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

    pub fn add_point(mut self, point: Point) -> Self {
        self.min = Point::new(
            f64::min(self.min.x(), point.x()),
            f64::min(self.min.y(), point.y()),
            f64::min(self.min.z(), point.z()),
        );

        self.max = Point::new(
            f64::max(self.max.x(), point.x()),
            f64::max(self.max.y(), point.y()),
            f64::max(self.max.z(), point.z()),
        );

        self
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
        let p1 = self.min;
        let p2 = Point::new(self.min.x(), self.min.y(), self.max.z());
        let p3 = Point::new(self.min.x(), self.max.y(), self.min.z());
        let p4 = Point::new(self.min.x(), self.max.y(), self.max.z());
        let p5 = Point::new(self.max.x(), self.min.y(), self.min.z());
        let p6 = Point::new(self.max.x(), self.min.y(), self.max.z());
        let p7 = Point::new(self.max.x(), self.max.y(), self.min.z());
        let p8 = self.max;

        Self::default()
            .add_point(&transformation.matrix * &p1)
            .add_point(&transformation.matrix * &p2)
            .add_point(&transformation.matrix * &p3)
            .add_point(&transformation.matrix * &p4)
            .add_point(&transformation.matrix * &p5)
            .add_point(&transformation.matrix * &p6)
            .add_point(&transformation.matrix * &p7)
            .add_point(&transformation.matrix * &p8)
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
        self.add_point(rhs.min).add_point(rhs.max)
    }
}
