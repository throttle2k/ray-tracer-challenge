use colo_rs::colors::Color;

use crate::{
    transformations::Transformation,
    tuples::{points::Point, Tuple},
};

use super::Pattern;

#[derive(Debug, Clone, PartialEq)]
pub struct StripePattern {
    a: Box<Pattern>,
    b: Box<Pattern>,
    transform: Transformation,
}

impl StripePattern {
    pub fn new(a: Pattern, b: Pattern) -> Self {
        Self {
            a: Box::new(a),
            b: Box::new(b),
            transform: Transformation::new_transform(),
        }
    }

    pub fn with_transform(mut self, t: Transformation) -> Self {
        self.transform = t;
        self
    }

    pub fn pattern_at(&self, p: Point) -> Color {
        if p.x().floor() % 2.0 == 0.0 {
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
    fn creating_a_stripe_pattern() {
        let pattern = StripePattern::new(Pattern::new_test_pattern(), Pattern::new_test_pattern());
        assert_eq!(pattern.a, Box::new(Pattern::new_test_pattern()));
        assert_eq!(pattern.b, Box::new(Pattern::new_test_pattern()));
    }

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = StripePattern::new(
            Pattern::new_solid_pattern(Color::white()),
            Pattern::new_solid_pattern(Color::black()),
        );
        assert_eq!(
            pattern.pattern_at(Point::new(0.0, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(Point::new(0.0, 1.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(Point::new(0.0, 2.0, 0.0)),
            Color::white()
        );
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = StripePattern::new(
            Pattern::new_solid_pattern(Color::white()),
            Pattern::new_solid_pattern(Color::black()),
        );
        assert_eq!(
            pattern.pattern_at(Point::new(0.0, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(Point::new(0.0, 0.0, 1.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(Point::new(0.0, 0.0, 2.0)),
            Color::white()
        );
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = StripePattern::new(
            Pattern::new_solid_pattern(Color::white()),
            Pattern::new_solid_pattern(Color::black()),
        );
        assert_eq!(
            pattern.pattern_at(Point::new(0.0, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(Point::new(0.9, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(Point::new(1.0, 0.0, 0.0)),
            Color::black()
        );
        assert_eq!(
            pattern.pattern_at(Point::new(-0.1, 0.0, 0.0)),
            Color::black()
        );
        assert_eq!(
            pattern.pattern_at(Point::new(-1.0, 0.0, 0.0)),
            Color::black()
        );
        assert_eq!(
            pattern.pattern_at(Point::new(-1.1, 0.0, 0.0)),
            Color::white()
        );
    }
}
