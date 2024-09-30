pub mod blending_pattern;
pub mod checker_pattern;
pub mod linear_gradient_pattern;
pub mod perturbed_pattern;
pub mod ring_pattern;
pub mod solid_pattern;
pub mod striped_pattern;
pub mod test_pattern;

use blending_pattern::BlendingPattern;
use checker_pattern::CheckerPattern;
use colo_rs::colors::Color;
use linear_gradient_pattern::LinearGradientPattern;
use perturbed_pattern::PerturbedPattern;
use ring_pattern::RingPattern;
use solid_pattern::SolidPattern;
use striped_pattern::StripePattern;
use test_pattern::TestPattern;

use crate::{shapes::Object, transformations::Transformation, tuples::points::Point};

#[derive(Debug, Clone, PartialEq)]
enum PatternType {
    Striped(StripePattern),
    Test(TestPattern),
    LinearGradient(LinearGradientPattern),
    Ring(RingPattern),
    Checker(CheckerPattern),
    Solid(SolidPattern),
    Blending(BlendingPattern),
    Perturbed(PerturbedPattern),
}

impl PatternType {
    fn pattern_at(&self, p: Point) -> Color {
        match self {
            PatternType::Striped(pattern) => pattern.pattern_at(p),
            PatternType::Test(pattern) => pattern.pattern_at(p),
            PatternType::LinearGradient(pattern) => pattern.pattern_at(p),
            PatternType::Ring(pattern) => pattern.pattern_at(p),
            PatternType::Checker(pattern) => pattern.pattern_at(p),
            PatternType::Solid(pattern) => pattern.pattern_at(),
            PatternType::Blending(pattern) => pattern.pattern_at(p),
            PatternType::Perturbed(pattern) => pattern.pattern_at(p),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    pattern_type: PatternType,
    transform: Transformation,
}

impl Pattern {
    pub fn new_striped_pattern(a: Pattern, b: Pattern) -> Self {
        Self {
            pattern_type: PatternType::Striped(StripePattern::new(a, b)),
            transform: Transformation::new_transform(),
        }
    }

    pub fn new_test_pattern() -> Self {
        Self {
            pattern_type: PatternType::Test(TestPattern {}),
            transform: Transformation::new_transform(),
        }
    }

    pub fn new_linear_gradient(a: Color, b: Color) -> Self {
        Self {
            pattern_type: PatternType::LinearGradient(LinearGradientPattern::new(a, b)),
            transform: Transformation::new_transform(),
        }
    }

    pub fn new_ring_pattern(a: Pattern, b: Pattern) -> Self {
        Self {
            pattern_type: PatternType::Ring(RingPattern::new(a, b)),
            transform: Transformation::new_transform(),
        }
    }

    pub fn new_checker_pattern(a: Pattern, b: Pattern) -> Self {
        Self {
            pattern_type: PatternType::Checker(CheckerPattern::new(a, b)),
            transform: Transformation::new_transform(),
        }
    }

    pub fn new_solid_pattern(c: Color) -> Self {
        Self {
            pattern_type: PatternType::Solid(SolidPattern::new(c)),
            transform: Transformation::new_transform(),
        }
    }

    pub fn new_blending_pattern(a: Pattern, b: Pattern) -> Self {
        Self {
            pattern_type: PatternType::Blending(BlendingPattern::new(a, b)),
            transform: Transformation::new_transform(),
        }
    }

    pub fn new_perturbed_pattern(p: Pattern) -> Self {
        Self {
            pattern_type: PatternType::Perturbed(PerturbedPattern::new(p)),
            transform: Transformation::new_transform(),
        }
    }

    pub fn with_transform(mut self, t: Transformation) -> Self {
        self.transform = t;
        self
    }

    fn pattern_at(&self, p: Point) -> Color {
        let pattern_point = self.transform.inverse().unwrap() * &p;
        self.pattern_type.pattern_at(pattern_point)
    }

    pub fn pattern_at_object(&self, obj: &Object, p: Point) -> Color {
        let object_point = obj.to_object_space(&p).unwrap();
        self.pattern_at(object_point)
    }
}

#[cfg(test)]
mod tests {

    use crate::{matrix::Matrix, shapes::Object, transformations::Transformation, tuples::Tuple};

    use super::*;

    #[test]
    fn the_default_pattern_transform() {
        let pattern = Pattern::new_test_pattern();
        assert_eq!(pattern.transform.matrix, Matrix::identity(4));
    }

    #[test]
    fn assigning_a_transformation() {
        let pattern = Pattern::new_test_pattern()
            .with_transform(Transformation::new_transform().translation(1.0, 2.0, 3.0));
        assert_eq!(
            pattern.transform,
            Transformation::new_transform().translation(1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn a_pattern_with_an_object_transformation() {
        let object = Object::new_sphere()
            .with_transform(Transformation::new_transform().scaling(2.0, 2.0, 2.0));
        let pattern = Pattern::new_test_pattern();
        let c = pattern.pattern_at_object(&object, Point::new(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn a_pattern_with_a_pattern_transformation() {
        let object = Object::new_sphere();
        let pattern = Pattern::new_test_pattern()
            .with_transform(Transformation::new_transform().scaling(2.0, 2.0, 2.0));
        let c = pattern.pattern_at_object(&object, Point::new(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn a_pattern_with_both_an_object_and_a_pattern_transformation() {
        let object = Object::new_sphere()
            .with_transform(Transformation::new_transform().scaling(2.0, 2.0, 2.0));
        let pattern = Pattern::new_test_pattern()
            .with_transform(Transformation::new_transform().translation(0.5, 1.0, 1.5));
        let c = pattern.pattern_at_object(&object, Point::new(2.5, 3.0, 3.5));
        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }
}
