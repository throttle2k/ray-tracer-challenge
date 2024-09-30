use crate::tuples::{points::Point, Tuple};

use colo_rs::colors::Color;

use super::Pattern;

#[derive(Debug, Clone, PartialEq)]
pub struct RingPattern {
    a: Box<Pattern>,
    b: Box<Pattern>,
}

impl RingPattern {
    pub fn new(a: Pattern, b: Pattern) -> Self {
        Self {
            a: Box::new(a),
            b: Box::new(b),
        }
    }

    pub fn pattern_at(&self, p: Point) -> Color {
        let x_squared = p.x() * p.x();
        let z_squared = p.z() * p.z();
        let distance = (x_squared + z_squared).sqrt();
        if distance.floor() % 2.0 == 0.0 {
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
    fn a_ring_should_extend_in_both_x_and_z() {
        let pattern = RingPattern::new(
            Pattern::new_solid_pattern(Color::white()),
            Pattern::new_solid_pattern(Color::black()),
        );
        assert_eq!(pattern.pattern_at(Point::zero()), Color::white());
        assert_eq!(
            pattern.pattern_at(Point::new(1.0, 0.0, 0.0)),
            Color::black()
        );
        assert_eq!(
            pattern.pattern_at(Point::new(0.0, 0.0, 1.0)),
            Color::black()
        );
        assert_eq!(
            pattern.pattern_at(Point::new(0.708, 0.0, 0.708)),
            Color::black()
        );
    }
}
