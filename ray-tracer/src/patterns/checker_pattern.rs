use colo_rs::colors::Color;

use crate::tuples::{points::Point, Tuple};

use super::Pattern;

#[derive(Debug, Clone, PartialEq)]
pub struct CheckerPattern {
    a: Box<Pattern>,
    b: Box<Pattern>,
}

impl CheckerPattern {
    pub fn new(a: Pattern, b: Pattern) -> Self {
        Self {
            a: Box::new(a),
            b: Box::new(b),
        }
    }

    pub fn pattern_at(&self, p: Point) -> Color {
        let sum = p.x().floor() + p.y().floor() + p.z().floor();
        if sum % 2.0 == 0.0 {
            self.a.pattern_at(p)
        } else {
            self.b.pattern_at(p)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn checkers_should_repeat_in_x() {
        let pattern = CheckerPattern::new(
            Pattern::new_solid_pattern(Color::white()),
            Pattern::new_solid_pattern(Color::black()),
        );
        assert_eq!(pattern.pattern_at(Point::zero()), Color::white());
        assert_eq!(
            pattern.pattern_at(Point::new(0.99, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(Point::new(1.01, 0.0, 0.0)),
            Color::black()
        );
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = CheckerPattern::new(
            Pattern::new_solid_pattern(Color::white()),
            Pattern::new_solid_pattern(Color::black()),
        );
        assert_eq!(pattern.pattern_at(Point::zero()), Color::white());
        assert_eq!(
            pattern.pattern_at(Point::new(0.0, 0.99, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(Point::new(0.0, 1.01, 0.0)),
            Color::black()
        );
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = CheckerPattern::new(
            Pattern::new_solid_pattern(Color::white()),
            Pattern::new_solid_pattern(Color::black()),
        );
        assert_eq!(pattern.pattern_at(Point::zero()), Color::white());
        assert_eq!(
            pattern.pattern_at(Point::new(0.0, 0.0, 0.99)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(Point::new(0.0, 0.0, 1.01)),
            Color::black()
        );
    }
}
