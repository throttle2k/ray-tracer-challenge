use crate::{
    bounds::Bounds,
    tuples::{points::Point, vectors::Vector, Tuple},
};

pub struct TestShape {}

impl TestShape {
    pub fn normal_at(object_point: Point) -> Vector {
        Vector::new(object_point.x(), object_point.y(), object_point.z())
    }

    pub fn bounds() -> crate::bounds::Bounds {
        Bounds::new(Point::zero(), Point::zero())
    }
}
