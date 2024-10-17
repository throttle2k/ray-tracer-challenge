use approx_eq::ApproxEq;

use crate::{
    bounds::Bounds,
    intersections::{Intersection, Intersections},
    rays::Ray,
    tuples::{points::Point, vectors::Vector, Tuple},
};

use super::Object;

#[derive(Debug, Clone, PartialEq)]
pub struct SmoothTriangle {
    p1: Point,
    p2: Point,
    p3: Point,
    n1: Vector,
    n2: Vector,
    n3: Vector,
}

impl Default for SmoothTriangle {
    fn default() -> Self {
        let p1 = Point::new(0.0, 1.0, 0.0);
        let p2 = Point::new(-1.0, 0.0, 0.0);
        let p3 = Point::new(1.0, 0.0, 0.0);
        let n1 = Vector::new(0.0, 1.0, 0.0);
        let n2 = Vector::new(-1.0, 0.0, 0.0);
        let n3 = Vector::new(1.0, 0.0, 0.0);
        Self {
            p1,
            p2,
            p3,
            n1,
            n2,
            n3,
        }
    }
}

impl SmoothTriangle {
    pub fn normal_at(&self, _object_point: Point, hit: Intersection) -> Vector {
        self.n2 * hit.u.unwrap()
            + self.n3 * hit.v.unwrap()
            + self.n1 * (1.0 - hit.u.unwrap() - hit.v.unwrap())
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
                    xs.push(Intersection::new(t, object).with_uv(u, v));
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

    pub fn n1(&self) -> Vector {
        self.n1
    }

    pub fn n2(&self) -> Vector {
        self.n2
    }

    pub fn n3(&self) -> Vector {
        self.n3
    }

    pub fn set_n1(&mut self, n1: Vector) {
        self.n1 = n1;
    }

    pub fn set_n2(&mut self, n2: Vector) {
        self.n2 = n2;
    }

    pub fn set_n3(&mut self, n3: Vector) {
        self.n3 = n3;
    }

    pub fn e1(&self) -> Vector {
        self.p2 - self.p1
    }

    pub fn e2(&self) -> Vector {
        self.p3 - self.p1
    }
}

#[cfg(test)]
mod tests {
    use crate::shapes::ObjectBuilder;

    use super::*;

    fn default_smooth_triangle() -> Object {
        let p1 = Point::new(0.0, 1.0, 0.0);
        let p2 = Point::new(-1.0, 0.0, 0.0);
        let p3 = Point::new(1.0, 0.0, 0.0);
        let n1 = Vector::new(0.0, 1.0, 0.0);
        let n2 = Vector::new(-1.0, 0.0, 0.0);
        let n3 = Vector::new(1.0, 0.0, 0.0);
        ObjectBuilder::new_smooth_triangle()
            .set_p1(p1)
            .set_p2(p2)
            .set_p3(p3)
            .set_n1(n1)
            .set_n2(n2)
            .set_n3(n3)
            .build()
    }

    #[test]
    fn constructing_a_smooth_triangle() {
        let p1 = Point::new(0.0, 1.0, 0.0);
        let p2 = Point::new(-1.0, 0.0, 0.0);
        let p3 = Point::new(1.0, 0.0, 0.0);
        let n1 = Vector::new(0.0, 1.0, 0.0);
        let n2 = Vector::new(-1.0, 0.0, 0.0);
        let n3 = Vector::new(1.0, 0.0, 0.0);
        let st = default_smooth_triangle();
        assert_eq!(st.p1().unwrap(), p1);
        assert_eq!(st.p2().unwrap(), p2);
        assert_eq!(st.p3().unwrap(), p3);
        assert_eq!(st.n1().unwrap(), n1);
        assert_eq!(st.n2().unwrap(), n2);
        assert_eq!(st.n3().unwrap(), n3);
    }

    #[test]
    fn an_intersection_with_a_smooth_triangle_stores_uv() {
        let tri = ObjectBuilder::new_smooth_triangle().build();
        let r = Ray::new(Point::new(-0.2, 0.3, -2.0), Vector::z_norm());
        let xs = tri.intersects(&r);
        assert_eq!(xs.len(), 1);
        assert!(xs[0].u.is_some());
        assert!(xs[0].v.is_some());
        assert!(xs[0].u.unwrap().approx_eq(0.45));
        assert!(xs[0].v.unwrap().approx_eq(0.25));
    }

    #[test]
    fn a_smooth_triangle_uses_uv_to_interpolate_normal() {
        let tri = ObjectBuilder::new_smooth_triangle().build();
        let shape = tri.shape.clone();
        let i = Intersection::new(1.0, &tri).with_uv(0.45, 0.25);
        match shape {
            crate::shapes::Shape::SmoothTriangle(s) => {
                let n = s.normal_at(Point::zero(), i).normalize();
                assert_eq!(n, Vector::new(-0.5547, 0.83205, 0.0));
            }
            _ => panic!("Wrong shape!"),
        }
    }

    #[test]
    fn preparing_the_normal_on_a_smooth_triangle() {
        let tri = ObjectBuilder::new_smooth_triangle().build();
        let r = Ray::new(Point::new(-0.2, 0.3, -2.0), Vector::z_norm());
        let xs = tri.intersects(&r);
        let i = Intersection::new(1.0, &tri).with_uv(0.45, 0.25);
        let comps = i.prepare_computations(r, &xs);
        assert_eq!(comps.normal_v, Vector::new(-0.5547, 0.83205, 0.0));
    }

    #[test]
    fn a_smooth_triangle_has_a_bounding_box() {
        let mut s = SmoothTriangle::default();
        s.set_p1(Point::new(-3.0, 7.0, 2.0));
        s.set_p2(Point::new(6.0, 2.0, -4.0));
        s.set_p3(Point::new(2.0, -1.0, -1.0));
        let b = s.bounds();
        assert_eq!(b.min(), &Point::new(-3.0, -1.0, -4.0));
        assert_eq!(b.max(), &Point::new(6.0, 7.0, 2.0));
    }
}
