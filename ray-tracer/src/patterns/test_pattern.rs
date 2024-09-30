use colo_rs::colors::Color;

use crate::tuples::{points::Point, Tuple};

#[derive(Debug, Clone, PartialEq)]
pub struct TestPattern {}

impl TestPattern {
    pub fn pattern_at(&self, p: Point) -> Color {
        Color::new(p.x(), p.y(), p.z())
    }
}
