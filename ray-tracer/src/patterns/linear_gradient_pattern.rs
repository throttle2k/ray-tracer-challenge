use colo_rs::colors::Color;

use crate::tuples::{points::Point, Tuple};

#[derive(Debug, Clone, PartialEq)]
pub struct LinearGradientPattern {
    a: Color,
    b: Color,
}

impl LinearGradientPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn pattern_at(&self, p: Point) -> Color {
        let distance = &self.b - &self.a;
        let fraction = p.x() - p.x().floor();
        &self.a + &(distance * fraction)
    }
}

#[cfg(test)]
mod tests {
    use colo_rs::colors::Color;

    use crate::tuples::{points::Point, Tuple};

    use super::*;

    #[test]
    fn a_gradient_linearly_interpolates_between_colors() {
        let pattern = LinearGradientPattern::new(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(Point::zero()), Color::white());
        assert_eq!(
            pattern.pattern_at(Point::new(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.pattern_at(Point::new(0.50, 0.0, 0.0)),
            Color::new(0.50, 0.50, 0.50)
        );
        assert_eq!(
            pattern.pattern_at(Point::new(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }
}
