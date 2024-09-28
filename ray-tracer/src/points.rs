use std::ops::{Add, Sub};

use crate::{tuples::Tuple, vectors::Vector};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Tuple for Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }

    fn w(&self) -> f64 {
        1.0
    }

    fn zero() -> Self {
        Point::new(0.0, 0.0, 0.0)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        let epsilon = 0.00001;
        (self.x - other.x) < epsilon && (self.y - other.y) < epsilon && (self.z - other.z) < epsilon
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Add<&Vector> for Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Self::Output {
        Point::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Add<Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Add<&Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Self::Output {
        Point::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        Vector::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Sub<Vector> for Point {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}
