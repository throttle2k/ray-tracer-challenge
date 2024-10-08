use approx_eq::ApproxEq;

use crate::{
    bounds::Bounds,
    intersections::{Intersection, Intersections},
    rays::Ray,
    tuples::{points::Point, vectors::Vector, Tuple},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
    e1: Vector,
    e2: Vector,
    normal: Vector,
}

impl Triangle {
    pub fn normal_at(&self, _object_point: Point) -> Vector {
        self.normal
    }

    pub fn intersects(&self, object_id: usize, r: Ray) -> Intersections {
        let dir_cross_e2 = r.direction * self.e2;
        let determinant = self.e1.dot(dir_cross_e2);
        let intersections = if determinant.approx_eq(0.0) {
            Intersections::new()
        } else {
            let f = 1.0 / determinant;
            let p1_to_origin = r.origin - self.p1;
            let u = f * p1_to_origin.dot(dir_cross_e2);
            if u < 0.0 || u > 1.0 {
                Intersections::new()
            } else {
                let origin_cross_e1 = p1_to_origin * self.e1;
                let v = f * r.direction.dot(origin_cross_e1);
                if v < 0.0 || (u + v) > 1.0 {
                    Intersections::new()
                } else {
                    let t = f * self.e2.dot(origin_cross_e1);
                    let mut xs = Intersections::new();
                    xs.push(Intersection::new(t, object_id));
                    xs
                }
            }
        };
        intersections
    }

    pub fn bounds(&self) -> Bounds {
        let min_x = self.p1.x().min(self.p2.x().min(self.p3.x()));
        let min_y = self.p1.y().min(self.p2.y().min(self.p3.y()));
        let min_z = self.p1.z().min(self.p2.z().min(self.p3.z()));
        let max_x = self.p1.x().max(self.p2.x().max(self.p3.x()));
        let max_y = self.p1.y().max(self.p2.y().max(self.p3.y()));
        let max_z = self.p1.z().max(self.p2.z().max(self.p3.z()));
        Bounds::new(
            Point::new(min_x, min_y, min_z),
            Point::new(max_x, max_y, max_z),
        )
    }

    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let normal = (e2 * e1).normalize();
        Self {
            p1,
            p2,
            p3,
            e1,
            e2,
            normal,
        }
    }

    pub fn p1(&self) -> Point {
        self.p1
    }

    pub fn p2(&self) -> Point {
        self.p2
    }

    pub fn p3(&self) -> Point {
        self.p3
    }

    pub fn e1(&self) -> Vector {
        self.e1
    }

    pub fn e2(&self) -> Vector {
        self.e2
    }

    pub fn normal(&self) -> Vector {
        self.normal
    }
}

#[cfg(test)]
mod tests {

    use crate::{shapes::ObjectBuilder, REGISTRY};

    use super::*;

    #[test]
    fn constructing_a_triangle() {
        let p1 = Point::new(0.0, 1.0, 0.0);
        let p2 = Point::new(-1.0, 0.0, 0.0);
        let p3 = Point::new(1.0, 0.0, 0.0);
        let t = ObjectBuilder::new_triangle(p1, p2, p3).register();
        let registry = REGISTRY.read().unwrap();
        let t = registry.get_object(t).unwrap();
        assert_eq!(t.p1(), p1);
        assert_eq!(t.p2(), p2);
        assert_eq!(t.p3(), p3);
        assert_eq!(t.e1(), Vector::new(-1.0, -1.0, 0.0));
        assert_eq!(t.e2(), Vector::new(1.0, -1.0, 0.0));
        assert_eq!(t.normal(), Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn finding_the_normal_of_a_triangle() {
        let t = ObjectBuilder::new_triangle(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        )
        .register();
        let registry = REGISTRY.read().unwrap();
        let t = registry.get_object(t).unwrap();
        let n1 = t.normal_at(Point::new(0.0, 0.5, 0.0));
        let n2 = t.normal_at(Point::new(-0.5, 0.75, 0.0));
        let n3 = t.normal_at(Point::new(0.5, 0.25, 0.0));
        assert_eq!(n1, t.normal());
        assert_eq!(n2, t.normal());
        assert_eq!(n3, t.normal());
    }

    #[test]
    fn intersecting_a_ray_parallel_to_the_triangle() {
        let t = ObjectBuilder::new_triangle(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        )
        .register();
        let registry = REGISTRY.read().unwrap();
        let t = registry.get_object(t).unwrap();
        let r = Ray::new(Point::new(0.0, -1.0, -2.0), Vector::y_norm());
        let xs = t.intersects(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_p1_p3_edge() {
        let t = ObjectBuilder::new_triangle(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        )
        .register();
        let registry = REGISTRY.read().unwrap();
        let t = registry.get_object(t).unwrap();
        let r = Ray::new(Point::new(1.0, 1.0, -2.0), Vector::z_norm());
        let xs = t.intersects(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_p1_p2_edge() {
        let t = ObjectBuilder::new_triangle(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        )
        .register();
        let registry = REGISTRY.read().unwrap();
        let t = registry.get_object(t).unwrap();
        let r = Ray::new(Point::new(-1.0, 1.0, -2.0), Vector::z_norm());
        let xs = t.intersects(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_p2_p3_edge() {
        let t = ObjectBuilder::new_triangle(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        )
        .register();
        let registry = REGISTRY.read().unwrap();
        let t = registry.get_object(t).unwrap();
        let r = Ray::new(Point::new(0.0, -1.0, -2.0), Vector::z_norm());
        let xs = t.intersects(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_strikes_a_triangle() {
        let t = ObjectBuilder::new_triangle(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        )
        .register();
        let registry = REGISTRY.read().unwrap();
        let t = registry.get_object(t).unwrap();
        let r = Ray::new(Point::new(0.0, 0.5, -2.0), Vector::z_norm());
        let xs = t.intersects(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 2.0);
    }
}
