use std::ops::{Add, Sub};

use approx_eq::ApproxEq;

use super::{vectors::Vector, Tuple};

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
        let both_x_same_sign = (self.x.is_sign_positive() && other.x.is_sign_positive())
            || (self.x.is_sign_negative() && other.x.is_sign_negative());
        if !both_x_same_sign {
            return false;
        }
        let both_y_same_sign = (self.y.is_sign_positive() && other.y.is_sign_positive())
            || (self.y.is_sign_negative() && other.y.is_sign_negative());
        if !both_y_same_sign {
            return false;
        }
        let both_z_same_sign = (self.z.is_sign_positive() && other.z.is_sign_positive())
            || (self.z.is_sign_negative() && other.z.is_sign_negative());
        if !both_z_same_sign {
            return false;
        }
        let both_x_finite = self.x.is_finite() && other.x.is_finite();
        let both_x_infinite = self.x.is_infinite() && other.x.is_infinite();
        if !(both_x_finite || both_x_infinite) {
            return false;
        }
        let both_y_finite = self.y.is_finite() && other.y.is_finite();
        let both_y_infinite = self.y.is_infinite() && other.y.is_infinite();
        if !(both_y_finite || both_y_infinite) {
            return false;
        }
        let both_z_finite = self.z.is_finite() && other.z.is_finite();
        let both_z_infinite = self.z.is_infinite() && other.z.is_infinite();
        if !(both_z_finite || both_z_infinite) {
            return false;
        }

        let x_eq = if both_x_finite {
            self.x.approx_eq(other.x)
        } else {
            true
        };

        let y_eq = if both_y_finite {
            self.y.approx_eq(other.y)
        } else {
            true
        };

        let z_eq = if both_z_finite {
            self.z.approx_eq(other.z)
        } else {
            true
        };

        x_eq && y_eq && z_eq
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
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl From<[f64; 3]> for Point {
    fn from(value: [f64; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}
