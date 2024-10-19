use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
};

use approx_eq::ApproxEq;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn red() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    pub fn green() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn blue() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn normalize(&mut self, min: f64, max: f64) {
        self.r = self.r.clamp(min, max);
        self.g = self.g.clamp(min, max);
        self.b = self.b.clamp(min, max);
    }

    pub fn round(&mut self) {
        self.r = self.r.round();
        self.g = self.g.round();
        self.b = self.b.round();
    }

    pub fn as_255_string(&self) -> String {
        let mut color_as_255 = self * 255.0;
        color_as_255.normalize(0.0, 255.0);
        color_as_255.round();

        format!("{} {} {}", color_as_255.r, color_as_255.g, color_as_255.b)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r.approx_eq(other.r) && self.g.approx_eq(other.g) && self.b.approx_eq(other.b)
    }
}

impl Add<&Color> for &Color {
    type Output = Color;

    fn add(self, rhs: &Color) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Sub<&Color> for &Color {
    type Output = Color;

    fn sub(self, rhs: &Color) -> Self::Output {
        Color::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        Color::new(self.r / rhs, self.g / rhs, self.b / rhs)
    }
}

impl Mul<f64> for &Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl Mul<&Color> for &Color {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        Color::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color::new(0.0, 0.0, 0.0), |acc, c| &acc + &c)
    }
}

impl From<[f64; 3]> for Color {
    fn from(value: [f64; 3]) -> Self {
        Self {
            r: value[0],
            g: value[1],
            b: value[2],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_can_be_created() {
        let color = Color::new(0.5, 0.4, 1.7);
        assert_eq!(color.r, 0.5);
        assert_eq!(color.g, 0.4);
        assert_eq!(color.b, 1.7);
    }

    #[test]
    fn color_can_be_added() {
        let color1 = Color::new(0.9, 0.6, 0.75);
        let color2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(&color1 + &color2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn color_can_be_subtracted() {
        let color1 = Color::new(0.9, 0.6, 0.75);
        let color2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(&color1 - &color2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn color_can_be_multiplied_by_a_scalar() {
        let color = Color::new(0.2, 0.3, 0.4);
        assert_eq!(color * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn color_can_be_multiplied() {
        let color1 = Color::new(1.0, 0.2, 0.4);
        let color2 = Color::new(0.9, 1.0, 0.1);
        assert_eq!(&color1 * &color2, Color::new(0.9, 0.2, 0.04));
    }
}
