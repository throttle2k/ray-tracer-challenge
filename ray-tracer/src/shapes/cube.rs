use approx_eq::ApproxEq;

use crate::{
    rays::Ray,
    tuples::{points::Point, vectors::Vector, Tuple},
};

pub struct Cube {}

impl Cube {
    pub fn normal_at(object_point: Point) -> Vector {
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

    fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
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

    pub fn intersects(ray: Ray) -> Vec<f64> {
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
        if tmin > tmax {
            Vec::new()
        } else {
            vec![tmin, tmax]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_ray_intersects_a_cube() {
        let examples = vec![
            (
                (Point::new(5.0, 0.5, 0.0), Vector::new(-1.0, 0.0, 0.0)),
                (4.0, 6.0),
            ),
            (
                (Point::new(-5.0, 0.5, 0.0), Vector::new(1.0, 0.0, 0.0)),
                (4.0, 6.0),
            ),
            (
                (Point::new(0.5, 5.0, 0.0), Vector::new(0.0, -1.0, 0.0)),
                (4.0, 6.0),
            ),
            (
                (Point::new(0.5, -5.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
                (4.0, 6.0),
            ),
            (
                (Point::new(0.5, 0.0, 5.0), Vector::new(0.0, 0.0, -1.0)),
                (4.0, 6.0),
            ),
            (
                (Point::new(0.5, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0)),
                (4.0, 6.0),
            ),
            (
                (Point::new(0.0, 0.5, 0.0), Vector::new(0.0, 0.0, 1.0)),
                (-1.0, 1.0),
            ),
        ];

        for example in examples {
            let r = Ray::new(example.0 .0, example.0 .1);
            let xs = Cube::intersects(r);
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0], example.1 .0);
            assert_eq!(xs[1], example.1 .1);
        }
    }

    #[test]
    fn a_ray_misses_a_cube() {
        let examples = vec![
            ((
                Point::new(-2.0, 0.0, 0.0),
                Vector::new(0.2673, 0.5345, 0.8018),
            ),),
            ((
                Point::new(0.0, -2.0, 0.0),
                Vector::new(0.8018, 0.2673, 0.5345),
            ),),
            ((
                Point::new(0.0, 0.0, -2.0),
                Vector::new(0.5345, 0.8018, 0.2673),
            ),),
            ((Point::new(2.0, 0.0, 2.0), Vector::new(0.0, 0.0, -1.0)),),
            ((Point::new(0.0, 2.0, 2.0), Vector::new(0.0, -1.0, 0.0)),),
            ((Point::new(2.0, 2.0, 0.0), Vector::new(-1.0, 0.0, 0.0)),),
        ];

        for example in examples {
            let r = Ray::new(example.0 .0, example.0 .1);
            let xs = Cube::intersects(r);
            assert!(xs.is_empty());
        }
    }

    #[test]
    fn the_normal_of_the_surface_of_a_cube() {
        let examples = vec![
            (Point::new(1.0, 0.5, -0.8), Vector::new(1.0, 0.0, 0.0)),
            (Point::new(-1.0, -0.2, 0.9), Vector::new(-1.0, 0.0, 0.0)),
            (Point::new(-0.4, 1.0, -0.1), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.3, -1.0, -0.7), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(-0.6, 0.3, 1.0), Vector::new(0.0, 0.0, 1.0)),
            (Point::new(0.4, 0.4, -1.0), Vector::new(0.0, 0.0, -1.0)),
            (Point::new(1.0, 1.0, 1.0), Vector::new(1.0, 0.0, 0.0)),
            (Point::new(-1.0, -1.0, -1.0), Vector::new(-1.0, 0.0, 0.0)),
        ];

        for example in examples {
            let normal = Cube::normal_at(example.0);
            assert_eq!(normal, example.1);
        }
    }
}
