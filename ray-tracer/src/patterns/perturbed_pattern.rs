use colo_rs::colors::Color;
use perlin_noise::PerlinNoise;

use crate::tuples::{points::Point, Tuple};

use super::Pattern;

#[derive(Debug, Clone, PartialEq)]
pub struct PerturbedPattern {
    pattern: Box<Pattern>,
}

impl PerturbedPattern {
    pub fn new(p: Pattern) -> Self {
        Self {
            pattern: Box::new(p),
        }
    }

    pub fn pattern_at(&self, p: Point) -> Color {
        let perlin = PerlinNoise::new();
        let perturbed_x = perlin.get3d([p.x(), p.y(), p.z()]) * 0.5;
        let perturbed_y = perlin.get3d([p.x(), p.y(), p.z()]) * 0.5;
        let perturbed_z = perlin.get3d([p.x(), p.y(), p.z()]) * 0.5;
        let new_p = Point::new(
            p.x() + perturbed_x,
            p.y() + perturbed_y,
            p.z() + perturbed_z,
        );
        self.pattern.pattern_at(new_p)
    }
}
