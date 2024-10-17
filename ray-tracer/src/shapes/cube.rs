use approx_eq::ApproxEq;

use crate::{
    bounds::Bounds,
    intersections::{Intersection, Intersections},
    rays::Ray,
    tuples::{points::Point, vectors::Vector, Tuple},
};

use super::Object;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Cube {}

impl Cube {
    pub fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;
        let tmin = tmin_numerator / direction;
        let tmax = tmax_numerator / direction;
        if tmin < tmax {
            (tmin, tmax)
        } else {
            (tmax, tmin)
        }
    }

    pub fn normal_at(&self, object_point: Point) -> Vector {
        let max_c = [
            object_point.x().abs(),
            object_point.y().abs(),
            object_point.z().abs(),
        ]
        .into_iter()
        .max_by(|a, b| a.total_cmp(b))
        .unwrap();
        if max_c.approx_eq(object_point.x().abs()) {
            Vector::new(object_point.x(), 0.0, 0.0)
        } else if max_c.approx_eq(object_point.y().abs()) {
            Vector::new(0.0, object_point.y(), 0.0)
        } else {
            Vector::new(0.0, 0.0, object_point.z())
        }
    }

    pub fn intersects<'a>(&self, object: &'a Object, ray: &Ray) -> Intersections<'a> {
        let (xtmin, xtmax) = Cube::check_axis(ray.origin.x(), ray.direction.x());
        let (ytmin, ytmax) = Cube::check_axis(ray.origin.y(), ray.direction.y());
        let (ztmin, ztmax) = Cube::check_axis(ray.origin.z(), ray.direction.z());
        let tmin = [xtmin, ytmin, ztmin]
            .into_iter()
            .max_by(|a, b| a.total_cmp(b))
            .unwrap();
        let tmax = [xtmax, ytmax, ztmax]
            .into_iter()
            .min_by(|a, b| a.total_cmp(b))
            .unwrap();
        let mut intersections = Intersections::new();
        if tmin <= tmax {
            intersections.push(Intersection::new(tmin, object));
            intersections.push(Intersection::new(tmax, object));
        }
        intersections
    }

    pub fn bounds(&self) -> Bounds {
        Bounds::new(Point::new(-1.0, -1.0, -1.0), Point::new(1.0, 1.0, 1.0))
    }
}

#[cfg(test)]
mod tests {
    use yare::parameterized;

    use crate::shapes::ObjectBuilder;

    use super::*;

    #[parameterized(
        strike_along_x_neg_axis = {Point::new(5.0, 0.5, 0.0), Vector::new(-1.0, 0.0, 0.0), 4.0, 6.0},
        strike_along_x_axis = {Point::new(-5.0, 0.5, 0.0), Vector::new(1.0, 0.0, 0.0), 4.0, 6.0},
        strike_along_y_neg_axis = {Point::new(0.5, 5.0, 0.0), Vector::new(0.0, -1.0, 0.0), 4.0, 6.0},
        strike_along_y_axis = {Point::new(0.5, -5.0, 0.0), Vector::new(0.0, 1.0, 0.0), 4.0, 6.0},
        strike_along_z_neg_axis = {Point::new(0.5, 0.0, 5.0), Vector::new(0.0, 0.0, -1.0), 4.0, 6.0},
        strike_along_z_axis = {Point::new(0.5, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0), 4.0, 6.0},
        strike_along_edge = {Point::new(0.0, 0.5, 0.0), Vector::new(0.0, 0.0, 1.0), -1.0, 1.0},
    )]
    fn a_ray_intersects_a_cube(origin: Point, direction: Vector, t0: f64, t1: f64) {
        let cube = ObjectBuilder::new_cube().build();
        let r = Ray::new(origin, direction);
        let xs = cube.intersects(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, t0);
        assert_eq!(xs[1].t, t1);
    }

    #[parameterized(
        miss_1 = {Point::new(-2.0, 0.0, 0.0), Vector::new(0.2673, 0.5345, 0.8018)},
        miss_2 = {Point::new(0.0, -2.0, 0.0), Vector::new(0.8018, 0.2673, 0.5345)},
        miss_3 = {Point::new(0.0, 0.0, -2.0), Vector::new(0.5345, 0.8018, 0.2673)},
        miss_along_z_axis = {Point::new(2.0, 0.0, 2.0), Vector::new(0.0, 0.0, -1.0)},
        miss_along_y_axis = {Point::new(0.0, 2.0, 2.0), Vector::new(0.0, -1.0, 0.0)},
        miss_along_x_axis = {Point::new(2.0, 2.0, 0.0), Vector::new(-1.0, 0.0, 0.0)},
    )]
    fn a_ray_misses_a_cube(origin: Point, direction: Vector) {
        let cube = ObjectBuilder::new_cube().build();
        let r = Ray::new(origin, direction);
        let xs = cube.intersects(&r);
        assert!(xs.is_empty());
    }

    #[parameterized(
        normal_on_side = {Point::new(1.0, 0.5, -0.8), Vector::new(1.0, 0.0, 0.0)},
        normal_on_other_side = {Point::new(-1.0, -0.2, 0.9), Vector::new(-1.0, 0.0, 0.0)},
        normal_on_top = {Point::new(-0.4, 1.0, -0.1), Vector::new(0.0, 1.0, 0.0)},
        normal_on_bottom = {Point::new(0.3, -1.0, -0.7), Vector::new(0.0, -1.0, 0.0)},
        normal_on_front = {Point::new(-0.6, 0.3, 1.0), Vector::new(0.0, 0.0, 1.0)},
        normal_on_back = {Point::new(0.4, 0.4, -1.0), Vector::new(0.0, 0.0, -1.0)},
        normal_on_corner = {Point::new(1.0, 1.0, 1.0), Vector::new(1.0, 0.0, 0.0)},
        normal_on_other_corner = {Point::new(-1.0, -1.0, -1.0), Vector::new(-1.0, 0.0, 0.0)},
    )]
    fn the_normal_of_the_surface_of_a_cube(point: Point, normal: Vector) {
        let n = Cube::normal_at(&Cube::default(), point);
        assert_eq!(n, normal);
    }

    #[test]
    fn a_cube_has_a_bounding_box() {
        let s = Cube::default();
        let b = s.bounds();
        assert_eq!(b.min(), &Point::new(-1.0, -1.0, -1.0));
        assert_eq!(b.max(), &Point::new(1.0, 1.0, 1.0));
    }
}
