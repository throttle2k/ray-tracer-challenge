use crate::tuples::points::Point;

use colo_rs::colors::Color;

use super::Pattern;

#[derive(Debug, Clone, PartialEq)]
pub struct BlendingPattern {
    a: Box<Pattern>,
    b: Box<Pattern>,
}

impl BlendingPattern {
    pub fn new(a: Pattern, b: Pattern) -> Self {
        Self {
            a: Box::new(a),
            b: Box::new(b),
        }
    }

    pub fn pattern_at(&self, p: Point) -> Color {
        let c1 = self.a.pattern_at(p);
        let c2 = self.b.pattern_at(p);
        (&c1 + &c2) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{transformations::Transformation, tuples::Tuple};

    use super::*;

    #[test]
    fn pattern_colors_are_averaged() {
        let pattern = BlendingPattern::new(
            Pattern::new_striped_pattern(
                Pattern::new_solid_pattern(Color::white()),
                Pattern::new_solid_pattern(Color::black()),
            ),
            Pattern::new_striped_pattern(
                Pattern::new_solid_pattern(Color::white()),
                Pattern::new_solid_pattern(Color::black()),
            )
            .with_transform(Transformation::new_transform().rotation_y(-PI / 2.0)),
        );
        assert_eq!(
            pattern.pattern_at(Point::new(0.5, 0.0, 0.5)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(Point::new(1.5, 0.0, 0.5)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.pattern_at(Point::new(0.5, 0.0, 1.5)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.pattern_at(Point::new(1.5, 0.0, 1.5)),
            Color::black()
        );
    }
}
