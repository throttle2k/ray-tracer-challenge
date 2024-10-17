use approx_eq::ApproxEq;

use crate::{
    bounds::Bounds,
    intersections::{Intersection, Intersections},
    rays::Ray,
    tuples::{points::Point, vectors::Vector, Tuple},
};

use super::Object;

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

impl Default for Triangle {
    fn default() -> Self {
        let p1 = Point::new(0.0, 1.0, 0.0);
        let p2 = Point::new(-1.0, 0.0, 0.0);
        let p3 = Point::new(1.0, 0.0, 0.0);
        Self { p1, p2, p3 }
    }
}

impl Triangle {
    pub fn normal(&self) -> Vector {
        (self.e2() * self.e1()).normalize()
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

    pub fn set_p1(&mut self, p1: Point) {
        self.p1 = p1;
    }

    pub fn set_p2(&mut self, p2: Point) {
        self.p2 = p2;
    }

    pub fn set_p3(&mut self, p3: Point) {
        self.p3 = p3;
    }

    pub fn e1(&self) -> Vector {
        self.p2 - self.p1
    }

    pub fn e2(&self) -> Vector {
        self.p3 - self.p1
    }

    pub fn normal_at(&self, _object_point: Point) -> Vector {
        (self.e2() * self.e1()).normalize()
    }

    pub fn intersects<'a>(&self, object: &'a Object, r: &Ray) -> Intersections<'a> {
        let dir_cross_e2 = r.direction * self.e2();
        let determinant = self.e1().dot(dir_cross_e2);
        let intersections = if determinant.approx_eq(0.0) {
            Intersections::new()
        } else {
            let f = 1.0 / determinant;
            let p1_to_origin = r.origin - self.p1;
            let u = f * p1_to_origin.dot(dir_cross_e2);
            if u < 0.0 || u > 1.0 {
                Intersections::new()
            } else {
                let origin_cross_e1 = p1_to_origin * self.e1();
                let v = f * r.direction.dot(origin_cross_e1);
                if v < 0.0 || (u + v) > 1.0 {
                    Intersections::new()
                } else {
                    let t = f * self.e2().dot(origin_cross_e1);
                    let mut xs = Intersections::new();
                    xs.push(Intersection::new(t, object));
                    xs
                }
            }
        };
        intersections
    }

    pub fn bounds(&self) -> Bounds {
        let mut b = Bounds::default();
        b = b + &self.p1();
        b = b + &self.p2();
        b = b + &self.p3();
        b
    }
}

#[cfg(test)]
mod tests {

    use crate::shapes::ObjectBuilder;

    use super::*;

    #[test]
    fn constructing_a_triangle() {
        let p1 = Point::new(0.0, 1.0, 0.0);
        let p2 = Point::new(-1.0, 0.0, 0.0);
        let p3 = Point::new(1.0, 0.0, 0.0);
        let t = ObjectBuilder::new_triangle()
            .set_p1(p1)
            .set_p2(p2)
            .set_p3(p3)
            .build();
        assert_eq!(t.p1().unwrap(), p1);
        assert_eq!(t.p2().unwrap(), p2);
        assert_eq!(t.p3().unwrap(), p3);
        assert_eq!(t.e1().unwrap(), Vector::new(-1.0, -1.0, 0.0));
        assert_eq!(t.e2().unwrap(), Vector::new(1.0, -1.0, 0.0));
        assert_eq!(t.normal().unwrap(), Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn finding_the_normal_of_a_triangle() {
        let t = ObjectBuilder::new_triangle()
            .set_p1(Point::new(0.0, 1.0, 0.0))
            .set_p2(Point::new(-1.0, 0.0, 0.0))
            .set_p3(Point::new(1.0, 0.0, 0.0))
            .build();
        let n1 = t.normal_at(Point::new(0.0, 0.5, 0.0), Intersection::new(1.0, &t));
        let n2 = t.normal_at(Point::new(-0.5, 0.75, 0.0), Intersection::new(1.0, &t));
        let n3 = t.normal_at(Point::new(0.5, 0.25, 0.0), Intersection::new(1.0, &t));
        assert_eq!(n1, t.normal().unwrap());
        assert_eq!(n2, t.normal().unwrap());
        assert_eq!(n3, t.normal().unwrap());
    }

    #[test]
    fn intersecting_a_ray_parallel_to_the_triangle() {
        let t = ObjectBuilder::new_triangle()
            .set_p1(Point::new(0.0, 1.0, 0.0))
            .set_p2(Point::new(-1.0, 0.0, 0.0))
            .set_p3(Point::new(1.0, 0.0, 0.0))
            .build();
        let r = Ray::new(Point::new(0.0, -1.0, -2.0), Vector::y_norm());
        let xs = t.intersects(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_p1_p3_edge() {
        let t = ObjectBuilder::new_triangle()
            .set_p1(Point::new(0.0, 1.0, 0.0))
            .set_p2(Point::new(-1.0, 0.0, 0.0))
            .set_p3(Point::new(1.0, 0.0, 0.0))
            .build();
        let r = Ray::new(Point::new(1.0, 1.0, -2.0), Vector::z_norm());
        let xs = t.intersects(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_p1_p2_edge() {
        let t = ObjectBuilder::new_triangle()
            .set_p1(Point::new(0.0, 1.0, 0.0))
            .set_p2(Point::new(-1.0, 0.0, 0.0))
            .set_p3(Point::new(1.0, 0.0, 0.0))
            .build();
        let r = Ray::new(Point::new(-1.0, 1.0, -2.0), Vector::z_norm());
        let xs = t.intersects(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_p2_p3_edge() {
        let t = ObjectBuilder::new_triangle()
            .set_p1(Point::new(0.0, 1.0, 0.0))
            .set_p2(Point::new(-1.0, 0.0, 0.0))
            .set_p3(Point::new(1.0, 0.0, 0.0))
            .build();
        let r = Ray::new(Point::new(0.0, -1.0, -2.0), Vector::z_norm());
        let xs = t.intersects(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_strikes_a_triangle() {
        let t = ObjectBuilder::new_triangle()
            .set_p1(Point::new(0.0, 1.0, 0.0))
            .set_p2(Point::new(-1.0, 0.0, 0.0))
            .set_p3(Point::new(1.0, 0.0, 0.0))
            .build();
        let r = Ray::new(Point::new(0.0, 0.5, -2.0), Vector::z_norm());
        let xs = t.intersects(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 2.0);
    }

    #[test]
    fn a_triangle_has_a_bounding_box() {
        let mut s = Triangle::default();
        s.set_p1(Point::new(-3.0, 7.0, 2.0));
        s.set_p2(Point::new(6.0, 2.0, -4.0));
        s.set_p3(Point::new(2.0, -1.0, -1.0));
        let b = s.bounds();
        assert_eq!(b.min(), &Point::new(-3.0, -1.0, -4.0));
        assert_eq!(b.max(), &Point::new(6.0, 7.0, 2.0));
    }
}
