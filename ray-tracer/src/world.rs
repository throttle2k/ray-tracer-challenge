use colo_rs::colors::Color;

use crate::{
    intersections::{Computation, Intersections},
    lights::PointLight,
    materials::Material,
    rays::Ray,
    sphere::Sphere,
    transformations::Transformation,
    tuples::points::Point,
    tuples::Tuple,
};

#[derive(Debug)]
pub struct World {
    lights: Vec<PointLight>,
    objects: Vec<Sphere>,
}

impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let lights = vec![light];
        let m1 = Material::new()
            .with_color(Color::new(0.8, 1.0, 0.6))
            .with_diffuse(0.7)
            .with_specular(0.2);
        let s1 = Sphere::new().with_material(m1);
        let t2 = Transformation::new_transform().scaling(0.5, 0.5, 0.5);
        let s2 = Sphere::new().with_transform(t2);
        let objects = vec![s1, s2];
        Self { lights, objects }
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            lights: Vec::new(),
            objects: Vec::new(),
        }
    }

    pub fn with_lights(mut self, lights: Vec<PointLight>) -> Self {
        self.lights = lights;
        self
    }

    pub fn with_objects(mut self, objects: Vec<Sphere>) -> Self {
        self.objects = objects;
        self
    }

    pub fn lights(&self) -> &[PointLight] {
        &self.lights
    }

    pub fn objects(&self) -> &[Sphere] {
        &self.objects
    }

    pub fn object_mut(&mut self, idx: usize) -> Option<&mut Sphere> {
        self.objects.get_mut(idx)
    }

    pub fn intersect_world(&self, ray: Ray) -> Intersections {
        let mut xs = Intersections::new();
        self.objects
            .iter()
            .for_each(|obj| xs.push_all(obj.intersect(&ray)));
        xs.sort_by(|i1, i2| i1.t.total_cmp(&i2.t));
        xs
    }

    pub fn shade_hit(&self, comps: Computation) -> Color {
        self.lights()
            .iter()
            .map(|light| {
                let in_shadow = self.is_shadowed(comps.over_point);
                comps.object.material().lighting(
                    *light,
                    comps.over_point,
                    comps.eye_v,
                    comps.normal_v,
                    in_shadow,
                )
            })
            .sum()
    }

    pub fn color_at(&self, r: Ray) -> Color {
        let xs = self.intersect_world(r);
        if let Some(hit) = xs.hit() {
            let comps = hit.prepare_computations(r);
            self.shade_hit(comps)
        } else {
            Color::black()
        }
    }

    pub fn is_shadowed(&self, p: Point) -> bool {
        for light in self.lights() {
            let v = light.position - p;
            let distance = v.magnitude();
            let direction = v.normalize();
            let shadow_ray = Ray::new(p, direction);
            let xs = self.intersect_world(shadow_ray);
            let h = xs.hit();
            if let Some(h) = h {
                if h.t < distance {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {

    use crate::{intersections::Intersection, tuples::vectors::Vector};

    use super::*;

    #[test]
    fn creating_a_world() {
        let w = World::new();
        assert_eq!(w.objects(), Vec::new());
        assert_eq!(w.lights(), Vec::new());
    }

    #[test]
    fn default_world() {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let m1 = Material::new()
            .with_color(Color::new(0.8, 1.0, 0.6))
            .with_diffuse(0.7)
            .with_specular(0.2);
        let s1 = Sphere::new().with_material(m1);
        let t2 = Transformation::new_transform().scaling(0.5, 0.5, 0.5);
        let s2 = Sphere::new().with_transform(t2);
        let w = World::default();
        assert!(w.lights.contains(&light));
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
    }

    #[test]
    fn intersect_the_world_with_a_ray() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = w.intersect_world(r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::y_norm());
        let c = w.color_at(r);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_norm());
        let c = w.color_at(r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let mut w = World::default();
        let outer = w.object_mut(0).unwrap();
        outer.material_mut().ambient = 1.0;
        let inner = w.object_mut(1).unwrap();
        inner.material_mut().ambient = 1.0;
        let inner_color = inner.material().color;
        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let c = w.color_at(r);
        assert_eq!(c, inner_color);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default();
        let p = Point::new(0.0, 10.0, 0.0);
        assert_eq!(w.is_shadowed(p), false);
    }

    #[test]
    fn the_shadow_when_an_object_is_between_point_and_light() {
        let w = World::default();
        let p = Point::new(10.0, -10.0, 10.0);
        assert_eq!(w.is_shadowed(p), true);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = World::default();
        let p = Point::new(-20.0, 20.0, -20.0);
        assert_eq!(w.is_shadowed(p), false);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = World::default();
        let p = Point::new(-2.0, 2.0, -2.0);
        assert_eq!(w.is_shadowed(p), false);
    }

    #[test]
    fn shade_hit_with_intersection_in_shadow() {
        let l = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let s1 = Sphere::new();
        let t2 = Transformation::new_transform().translation(0.0, 0.0, 10.0);
        let s2 = Sphere::new().with_transform(t2);
        let w = World::new()
            .with_lights(vec![l])
            .with_objects(vec![s1, s2.clone()]);
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::z_norm());
        let i = Intersection::new(4.0, &s2);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }
}
