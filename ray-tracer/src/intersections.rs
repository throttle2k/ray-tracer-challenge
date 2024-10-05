use std::ops::{Deref, DerefMut, Index};

use approx_eq::EPSILON;

use crate::{
    rays::Ray,
    tuples::{points::Point, vectors::Vector},
    REGISTRY,
};

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub t: f64,
    pub object_id: usize,
}

#[derive(Debug)]
pub struct Computation {
    pub t: f64,
    pub object_id: usize,
    pub point: Point,
    pub over_point: Point,
    pub under_point: Point,
    pub eye_v: Vector,
    pub normal_v: Vector,
    pub inside: bool,
    pub reflect_v: Vector,
    pub n1: f64,
    pub n2: f64,
}

impl Computation {
    pub fn schlick(&self) -> f64 {
        let mut cos = self.eye_v.dot(self.normal_v);
        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));
            if sin2_t > 1.0 {
                return 1.0;
            }
            let cos_t = f64::sqrt(1.0 - sin2_t);
            cos = cos_t;
        }
        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl Intersection {
    pub fn new(t: f64, object_id: usize) -> Self {
        Self { t, object_id }
    }

    pub fn prepare_computations(&self, r: Ray, xs: &Intersections) -> Computation {
        let t = self.t;
        let object_id = self.object_id;
        let point = r.position(t);
        let eye_v = -r.direction;
        let registry = REGISTRY.read().unwrap();
        let object = registry.get_object(object_id).unwrap();
        let mut normal_v = object.normal_at(point);
        let inside = normal_v.dot(eye_v) < 0.0;
        if inside {
            normal_v = -normal_v;
        }
        let over_point = point + normal_v * EPSILON;
        let under_point = point - normal_v * EPSILON;
        let reflect_v = r.direction.reflect(normal_v);

        let mut n1 = 0.0;
        let mut n2 = 0.0;
        let mut containers: Vec<usize> = Vec::new();
        for x in xs.iter() {
            if x == self {
                if containers.is_empty() {
                    n1 = 1.0;
                } else {
                    let obj = registry.get_object(*containers.last().unwrap()).unwrap();
                    n1 = obj.material().refractive_index;
                }
            };

            if containers.contains(&x.object_id) {
                containers.retain(|o| *o != x.object_id);
            } else {
                containers.push(x.object_id);
            }

            if x == self {
                if containers.is_empty() {
                    n2 = 1.0;
                } else {
                    let obj = registry.get_object(*containers.last().unwrap()).unwrap();
                    n2 = obj.material().refractive_index;
                }
                break;
            };
        }

        Computation {
            t,
            object_id,
            point,
            over_point,
            under_point,
            eye_v,
            normal_v,
            inside,
            reflect_v,
            n1,
            n2,
        }
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.object_id == other.object_id
    }
}

#[derive(Debug)]
pub struct Intersections {
    intersections: Vec<Intersection>,
}

impl Intersections {
    pub fn new() -> Self {
        Self {
            intersections: Vec::new(),
        }
    }

    pub fn push(&mut self, i: Intersection) {
        self.intersections.push(i);
    }

    pub fn push_all(&mut self, xs: Intersections) {
        xs.iter().for_each(|i| self.intersections.push(*i));
    }

    pub fn hit(&self) -> Option<&Intersection> {
        self.intersections
            .iter()
            .filter(|i| i.t > 0.0)
            .min_by(|i, j| i.t.total_cmp(&j.t))
    }
}

impl Deref for Intersections {
    type Target = Vec<Intersection>;

    fn deref(&self) -> &Self::Target {
        &self.intersections
    }
}

impl DerefMut for Intersections {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.intersections
    }
}

impl Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}

#[cfg(test)]
mod tests {
    use approx_eq::{ApproxEq, EPSILON};
    use colo_rs::colors::Color;

    use crate::{
        lights::PointLight, materials::Material, shapes::ObjectBuilder,
        transformations::Transformation, tuples::Tuple, world::World,
    };

    use super::*;

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = ObjectBuilder::new_sphere().register();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
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
        let s = ObjectBuilder::new_sphere().register();
        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(1.0, s);
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
        let s = ObjectBuilder::new_sphere().register();
        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(-2.0, s);
        let mut xs = Intersections::new();
        xs.push(i2);
        xs.push(i1);
        let i = xs.hit();
        assert!(i.is_none());
    }

    #[test]
    fn hit_is_always_the_lowest_non_negative_intersection() {
        let s = ObjectBuilder::new_sphere().register();
        let i1 = Intersection::new(5.0, s);
        let i2 = Intersection::new(7.0, s);
        let i3 = Intersection::new(-3.0, s);
        let i4 = Intersection::new(2.0, s);
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
        let s = ObjectBuilder::new_sphere().register();
        let i = Intersection::new(4.0, s);
        let mut xs = Intersections::new();
        xs.push(i);
        let comps = i.prepare_computations(r, &xs);
        assert_eq!(comps.object_id, i.object_id);
        assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(comps.eye_v, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normal_v, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_when_intersection_occurs_outside() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_norm());
        let s = ObjectBuilder::new_sphere().register();
        let i = Intersection::new(4.0, s);
        let mut xs = Intersections::new();
        xs.push(i);
        let comps = i.prepare_computations(r, &xs);
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn hit_when_intersection_occurs_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::z_norm());
        let s = ObjectBuilder::new_sphere().register();
        let i = Intersection::new(1.0, s);
        let mut xs = Intersections::new();
        xs.push(i);
        let comps = i.prepare_computations(r, &xs);
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
        let i = Intersection::new(4.0, *shape);
        let mut xs = Intersections::new();
        xs.push(i);
        let comps = i.prepare_computations(r, &xs);
        let c = w.shade_hit(comps, 5);
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
        let i = Intersection::new(0.5, *shape);
        let mut xs = Intersections::new();
        xs.push(i);
        let comps = i.prepare_computations(r, &xs);
        let c = w.shade_hit(comps, 5);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_norm());
        let t = Transformation::new_transform().translation(0.0, 0.0, 1.0);
        let shape = ObjectBuilder::new_sphere().with_transform(t).register();
        let i = Intersection::new(5.0, shape);
        let mut xs = Intersections::new();
        xs.push(i);
        let comps = i.prepare_computations(r, &xs);
        assert!(comps.over_point.z() < -EPSILON / 2.0);
        assert!(comps.point.z() > comps.over_point.z());
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let shape = ObjectBuilder::new_plane().register();
        let r = Ray::new(
            Point::new(0.0, 1.0, -1.0),
            Vector::new(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let i = Intersection::new(f64::sqrt(2.0), shape);
        let mut xs = Intersections::new();
        xs.push(i);
        let comps = i.prepare_computations(r, &xs);
        assert_eq!(
            comps.reflect_v,
            Vector::new(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0)
        );
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let a = ObjectBuilder::new_glass_sphere()
            .with_transform(Transformation::new_transform().scaling(2.0, 2.0, 2.0))
            .with_material(Material::new().with_refractive_index(1.5))
            .register();
        let b = ObjectBuilder::new_glass_sphere()
            .with_transform(Transformation::new_transform().translation(0.0, 0.0, -0.25))
            .with_material(Material::new().with_refractive_index(2.0))
            .register();
        let c = ObjectBuilder::new_glass_sphere()
            .with_transform(Transformation::new_transform().translation(0.0, 0.0, 0.25))
            .with_material(Material::new().with_refractive_index(2.5))
            .register();
        let r = Ray::new(Point::new(0.0, 0.0, -4.0), Vector::z_norm());
        let mut xs = Intersections::new();
        xs.push(Intersection::new(2.0, a));
        xs.push(Intersection::new(2.75, b));
        xs.push(Intersection::new(3.25, c));
        xs.push(Intersection::new(4.75, b));
        xs.push(Intersection::new(5.25, c));
        xs.push(Intersection::new(6.0, a));
        for (id, x) in xs.iter().enumerate() {
            let comps = x.prepare_computations(r, &xs);
            match id {
                0 => {
                    assert_eq!(comps.n1, 1.0);
                    assert_eq!(comps.n2, 1.5);
                }
                1 => {
                    assert_eq!(comps.n1, 1.5);
                    assert_eq!(comps.n2, 2.0);
                }
                2 => {
                    assert_eq!(comps.n1, 2.0);
                    assert_eq!(comps.n2, 2.5);
                }
                3 => {
                    assert_eq!(comps.n1, 2.5);
                    assert_eq!(comps.n2, 2.5);
                }
                4 => {
                    assert_eq!(comps.n1, 2.5);
                    assert_eq!(comps.n2, 1.5);
                }
                5 => {
                    assert_eq!(comps.n1, 1.5);
                    assert_eq!(comps.n2, 1.0);
                }
                _ => panic!("Too many indexes"),
            }
        }
    }

    #[test]
    fn the_under_point_is_offset_below_the_surface() {
        let r = Ray::new(Point::new(0.0, 0.0, -0.5), Vector::z_norm());
        let shape = ObjectBuilder::new_glass_sphere()
            .with_transform(Transformation::new_transform().translation(0.0, 0.0, 1.0))
            .register();
        let i = Intersection::new(5.0, shape);
        let mut xs = Intersections::new();
        xs.push(i);
        let comps = i.prepare_computations(r, &xs);
        assert!(comps.under_point.z() > EPSILON / 2.0);
        assert!(comps.point.z() < comps.under_point.z());
    }

    #[test]
    fn the_shlick_approximation_under_total_internal_reflection() {
        let shape = ObjectBuilder::new_glass_sphere().register();
        let r = Ray::new(Point::new(0.0, 0.0, f64::sqrt(2.0) / 2.0), Vector::y_norm());
        let mut xs = Intersections::new();
        xs.push(Intersection::new(-f64::sqrt(2.0) / 2.0, shape));
        xs.push(Intersection::new(f64::sqrt(2.0) / 2.0, shape));
        let comps = xs[1].prepare_computations(r, &xs);
        let reflectance = comps.schlick();
        assert_eq!(reflectance, 1.0);
    }

    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let shape = ObjectBuilder::new_glass_sphere().register();
        let r = Ray::new(Point::zero(), Vector::y_norm());
        let mut xs = Intersections::new();
        xs.push(Intersection::new(-1.0, shape));
        xs.push(Intersection::new(1.0, shape));
        let comps = xs[1].prepare_computations(r, &xs);
        let reflectance = comps.schlick();
        assert!(reflectance.approx_eq(0.04));
    }

    #[test]
    fn the_schlick_approximation_with_small_angle_and_n2_gt_n1() {
        let shape = ObjectBuilder::new_glass_sphere().register();
        let r = Ray::new(Point::new(0.0, 0.99, -2.0), Vector::z_norm());
        let mut xs = Intersections::new();
        xs.push(Intersection::new(1.8589, shape));
        let comps = xs[0].prepare_computations(r, &xs);
        let reflectance = comps.schlick();
        assert!(reflectance.approx_eq(0.48873));
    }
}
