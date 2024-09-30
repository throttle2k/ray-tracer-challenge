use colo_rs::colors::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct SolidPattern {
    color: Color,
}

impl SolidPattern {
    pub fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn pattern_at(&self) -> Color {
        self.color
    }
}
