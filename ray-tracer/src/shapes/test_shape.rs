use crate::{
    bounds::Bounds,
    intersections::{Intersection, Intersections},
    rays::Ray,
    tuples::{points::Point, vectors::Vector, Tuple},
};

use super::Object;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TestShape {}

impl TestShape {
    pub fn normal_at(&self, object_point: Point) -> Vector {
        Vector::new(object_point.x(), object_point.y(), object_point.z())
    }

    pub fn bounds(&self) -> Bounds {
        Bounds::new(Point::new(-1.0, -1.0, -1.0), Point::new(1.0, 1.0, 1.0))
    }

    pub fn intersects<'a>(&self, object: &'a Object, _r: &Ray) -> Intersections<'a> {
        let mut xs = Intersections::new();
        xs.push(Intersection::new(1.0, object));
        xs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_has_a_arbitrary_bounding_box() {
        let s = TestShape::default();
        let b = s.bounds();
        assert_eq!(b.min(), &Point::new(-1.0, -1.0, -1.0));
        assert_eq!(b.max(), &Point::new(1.0, 1.0, 1.0));
    }
}
