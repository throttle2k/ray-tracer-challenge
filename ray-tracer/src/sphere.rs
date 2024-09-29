use crate::tuples::{points::Point, vectors::Vector, Tuple};

#[derive(Debug, Clone)]
pub struct Sphere {}

impl Sphere {
    pub fn normal_at(object_point: Point) -> Vector {
        object_point - Point::zero()
    }
}
