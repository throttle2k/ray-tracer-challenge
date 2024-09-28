use std::ops::{Deref, DerefMut, Index};

use crate::{points::Point, rays::Ray, sphere::Sphere, vectors::Vector};

#[derive(Debug, Clone, Copy)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere,
}

#[derive(Debug)]
pub struct Computation<'a> {
    pub t: f64,
    pub object: &'a Sphere,
    pub point: Point,
    pub eye_v: Vector,
    pub normal_v: Vector,
    pub inside: bool,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Sphere) -> Self {
        Self { t, object }
    }

    pub fn prepare_computations(&self, r: Ray) -> Computation {
        let t = self.t;
        let object = &self.object;
        let point = r.position(t);
        let eye_v = -r.direction;
        let mut normal_v = object.normal_at(point);
        let inside = normal_v.dot(eye_v) < 0.0;
        if inside {
            normal_v = -normal_v;
        }

        Computation {
            t,
            object,
            point,
            eye_v,
            normal_v,
            inside,
        }
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.object == other.object
    }
}

pub struct Intersections<'a> {
    intersections: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub fn new() -> Self {
        Self {
            intersections: Vec::new(),
        }
    }

    pub fn push(&mut self, i: Intersection<'a>) {
        self.intersections.push(i);
    }

    pub fn push_all(&mut self, xs: Intersections<'a>) {
        xs.iter().for_each(|i| self.intersections.push(*i));
    }

    pub fn hit(&self) -> Option<&Intersection> {
        self.intersections
            .iter()
            .filter(|i| i.t > 0.0)
            .min_by(|i, j| i.t.total_cmp(&j.t))
    }
}

impl<'a> Deref for Intersections<'a> {
    type Target = Vec<Intersection<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.intersections
    }
}

impl<'a> DerefMut for Intersections<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.intersections
    }
}

impl<'a> Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}

#[cfg(test)]
mod tests {
    use colo_rs::colors::Color;

    use crate::{lights::PointLight, tuples::Tuple, world::World};

    use super::*;

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let mut xs = Intersections::new();
        xs.push(i2);
        xs.push(i1);
        let i = xs.hit();
        assert!(i.is_some());
        let i = i.unwrap();
        assert_eq!(i, &i1);
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let mut xs = Intersections::new();
        xs.push(i2);
        xs.push(i1);
        let i = xs.hit();
        assert!(i.is_some());
        let i = i.unwrap();
        assert_eq!(i, &i2);
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(-2.0, &s);
        let mut xs = Intersections::new();
        xs.push(i2);
        xs.push(i1);
        let i = xs.hit();
        assert!(i.is_none());
    }

    #[test]
    fn hit_is_always_the_lowest_non_negative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let mut xs = Intersections::new();
        xs.push(i1);
        xs.push(i2);
        xs.push(i3);
        xs.push(i4);
        let i = xs.hit();
        assert!(i.is_some());
        let i = i.unwrap();
        assert_eq!(i, &i4);
    }

    #[test]
    fn precomputing_state_of_an_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_norm());
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(r);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(comps.eye_v, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normal_v, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_when_intersection_occurs_outside() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_norm());
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(r);
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn hit_when_intersection_occurs_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::z_norm());
        let shape = Sphere::new();
        let i = Intersection::new(1.0, &shape);
        let comps = i.prepare_computations(r);
        assert_eq!(comps.inside, true);
        assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(comps.eye_v, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normal_v, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_norm());
        let shape = w.objects().first().unwrap();
        let i = Intersection::new(4.0, shape);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_inside() {
        let w = World::default().with_lights(vec![PointLight::new(
            Point::new(0.0, 0.25, 0.0),
            Color::new(1.0, 1.0, 1.0),
        )]);
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::z_norm());
        let shape = w.objects().get(1).unwrap();
        let i = Intersection::new(0.5, shape);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }
}
