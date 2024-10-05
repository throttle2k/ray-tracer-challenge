use approx_eq::ApproxEq;
use colo_rs::colors::Color;

use crate::{
    intersections::{Computation, Intersections},
    lights::PointLight,
    materials::Material,
    rays::Ray,
    shapes::ObjectBuilder,
    transformations::Transformation,
    tuples::{points::Point, Tuple},
    REGISTRY,
};

#[derive(Debug)]
pub struct World {
    lights: Vec<PointLight>,
    objects: Vec<usize>,
}

impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let lights = vec![light];
        let m1 = Material::new()
            .with_color(Color::new(0.8, 1.0, 0.6))
            .with_diffuse(0.7)
            .with_specular(0.2);
        let s1 = ObjectBuilder::new_sphere().with_material(m1).register();
        let t2 = Transformation::new_transform().scaling(0.5, 0.5, 0.5);
        let s2 = ObjectBuilder::new_sphere().with_transform(t2).register();
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

    pub fn with_objects(mut self, objects: Vec<usize>) -> Self {
        self.objects = objects;
        self
    }

    pub fn lights(&self) -> &[PointLight] {
        &self.lights
    }

    pub fn objects(&self) -> &[usize] {
        &self.objects
    }

    pub fn intersect_world(&self, ray: Ray) -> Intersections {
        let mut xs = Intersections::new();
        let registry = REGISTRY.read().unwrap();
        self.objects
            .iter()
            .map(|&id| registry.get_object(id).unwrap())
            .for_each(|obj| xs.push_all(obj.intersects(&ray)));
        xs.sort_by(|i1, i2| i1.t.total_cmp(&i2.t));
        xs
    }

    pub fn shade_hit(&self, comps: Computation, remaining: usize) -> Color {
        self.lights()
            .iter()
            .map(|light| {
                let registry = REGISTRY.read().unwrap();
                let obj = registry.get_object(comps.object_id).unwrap();
                let in_shadow =
                    obj.material().receive_shadows && self.is_shadowed(comps.over_point);
                let surface = obj.material().lighting(
                    *light,
                    comps.over_point,
                    comps.eye_v,
                    comps.normal_v,
                    in_shadow,
                    obj,
                );
                let reflected = self.reflected_color(&comps, remaining);
                let refracted = self.refracted_color(&comps, remaining);
                let ref_sum =
                    if obj.material().reflective > 0.0 && obj.material().transparency > 0.0 {
                        let reflectance = comps.schlick();
                        let reflected_mod = &reflected * reflectance;
                        let refracted_mod = &refracted * (1.0 - reflectance);
                        &reflected_mod + &refracted_mod
                    } else {
                        &reflected + &refracted
                    };
                &surface + &ref_sum
            })
            .sum()
    }

    pub fn color_at(&self, r: Ray, remaining: usize) -> Color {
        let xs = self.intersect_world(r);
        if let Some(hit) = xs.hit() {
            let comps = hit.prepare_computations(r, &xs);
            self.shade_hit(comps, remaining)
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
            let xs: Intersections = self.intersect_world(shadow_ray);
            let mut shadowing_xs: Intersections = Intersections::new();
            xs.iter()
                .filter(|i| {
                    let registry = REGISTRY.read().unwrap();
                    let obj = registry.get_object(i.object_id).unwrap();
                    obj.material().cast_shadows == true
                })
                .for_each(|i| shadowing_xs.push(*i));

            let h = shadowing_xs.hit();
            if let Some(h) = h {
                if h.t < distance {
                    return true;
                }
            }
        }
        false
    }

    pub fn reflected_color(&self, comps: &Computation, remaining: usize) -> Color {
        let registry = REGISTRY.read().unwrap();
        let obj = registry.get_object(comps.object_id).unwrap();
        if obj.material().reflective.approx_eq(0.0) || remaining == 0 {
            Color::black()
        } else {
            let reflected_ray = Ray::new(comps.over_point, comps.reflect_v);
            let color = self.color_at(reflected_ray, remaining - 1);
            color * obj.material().reflective
        }
    }

    pub fn refracted_color(&self, comps: &Computation, remaining: usize) -> Color {
        let registry = REGISTRY.read().unwrap();
        let obj = registry.get_object(comps.object_id).unwrap();
        if obj.material().transparency.approx_eq(0.0) || remaining == 0 {
            return Color::black();
        };
        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eye_v.dot(comps.normal_v);
        let sin2_t = n_ratio * n_ratio * (1.0 - cos_i * cos_i);
        if sin2_t > 1.0 {
            Color::black()
        } else {
            let cos_t = f64::sqrt(1.0 - sin2_t);
            let direction = comps.normal_v * (n_ratio * cos_i - cos_t) - comps.eye_v * n_ratio;
            let refract_ray = Ray::new(comps.under_point, direction);
            self.color_at(refract_ray, remaining - 1) * obj.material().transparency
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{intersections::Intersection, patterns::Pattern, tuples::vectors::Vector};

    use super::*;

    #[test]
    fn creating_a_world() {
        let w = World::new();
        assert_eq!(w.objects(), Vec::new());
        assert_eq!(w.lights(), Vec::new());
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
        let c = w.color_at(r, 5);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_norm());
        let c = w.color_at(r, 5);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let lights = vec![light];
        let s1 = ObjectBuilder::new_sphere()
            .with_material(
                Material::new()
                    .with_color(Color::new(0.8, 1.0, 0.6))
                    .with_diffuse(0.7)
                    .with_ambient(1.0)
                    .with_specular(0.2),
            )
            .register();
        let s2 = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().scaling(0.5, 0.5, 0.5))
            .with_material(Material::new().with_ambient(1.0))
            .register();
        let objects = vec![s1, s2];

        let w = World::new().with_objects(objects).with_lights(lights);

        let registry = REGISTRY.read().unwrap();
        let inner = registry.get_object(*w.objects().get(1).unwrap()).unwrap();
        let inner_color = inner.material().color;
        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let c = w.color_at(r, 5);
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
    fn a_material_can_opt_out_shadow() {
        let object_without_shadow = ObjectBuilder::new_plane()
            .with_material(Material::new().with_cast_shadows(false))
            .with_transform(Transformation::new_transform().translation(0.0, 1.0, 0.0))
            .register();
        let target_object = ObjectBuilder::new_plane().register();
        let light = PointLight::new(Point::new(0.0, 10.0, 0.0), Color::white());
        let w = World::new()
            .with_objects(vec![object_without_shadow, target_object])
            .with_lights(vec![light]);
        let p = Point::new(0.0, 0.0, 0.0);
        assert_eq!(w.is_shadowed(p), false);
    }

    #[test]
    fn shade_hit_with_intersection_in_shadow() {
        let l = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let s1 = ObjectBuilder::new_sphere().register();
        let t2 = Transformation::new_transform().translation(0.0, 0.0, 10.0);
        let s2 = ObjectBuilder::new_sphere().with_transform(t2).register();
        let w = World::new()
            .with_lights(vec![l])
            .with_objects(vec![s1, s2.clone()]);
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::z_norm());
        let i = Intersection::new(4.0, s2);
        let mut xs = Intersections::new();
        xs.push(i);
        let comps = i.prepare_computations(r, &xs);
        let c = w.shade_hit(comps, 5);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn a_non_reflective_material_is_black() {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let lights = vec![light];
        let s1 = ObjectBuilder::new_sphere()
            .with_material(
                Material::new()
                    .with_color(Color::new(0.8, 1.0, 0.6))
                    .with_diffuse(0.7)
                    .with_specular(0.2),
            )
            .register();
        let s2 = ObjectBuilder::new_sphere()
            .with_material(Material::new().with_ambient(1.0))
            .with_transform(Transformation::new_transform().scaling(0.5, 0.5, 0.5))
            .register();
        let objects = vec![s1, s2.clone()];
        let w = World::new().with_lights(lights).with_objects(objects);
        let r = Ray::new(Point::zero(), Vector::z_norm());
        let i = Intersection::new(1.0, s2);
        let mut xs = Intersections::new();
        xs.push(i);
        let comps = i.prepare_computations(r, &xs);
        let color = w.reflected_color(&comps, 5);
        assert_eq!(color, Color::black());
    }

    #[test]
    fn a_reflective_material_reflects_color() {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let lights = vec![light];
        let s1 = ObjectBuilder::new_sphere()
            .with_material(
                Material::new()
                    .with_color(Color::new(0.8, 1.0, 0.6))
                    .with_diffuse(0.7)
                    .with_specular(0.2),
            )
            .register();
        let s2 = ObjectBuilder::new_sphere().register();
        let shape = ObjectBuilder::new_plane()
            .with_material(Material::new().with_reflective(0.5))
            .with_transform(Transformation::new_transform().translation(0.0, -1.0, 0.0))
            .register();
        let objects = vec![s1, s2, shape];
        let w = World::new().with_lights(lights).with_objects(objects);
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let i = Intersection::new(f64::sqrt(2.0), shape);
        let mut xs = Intersections::new();
        xs.push(i);
        let comps = i.prepare_computations(r, &xs);
        let color = w.shade_hit(comps, 5);
        assert_eq!(color, Color::new(0.87676, 0.92434, 0.82917));
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let w = World::new()
            .with_lights(vec![PointLight::new(Point::zero(), Color::white())])
            .with_objects(vec![
                ObjectBuilder::new_plane()
                    .with_material(Material::new().with_reflective(1.0))
                    .with_transform(Transformation::new_transform().translation(0.0, -1.0, 0.0))
                    .register(),
                ObjectBuilder::new_plane()
                    .with_material(Material::new().with_reflective(1.0))
                    .with_transform(Transformation::new_transform().translation(0.0, 1.0, 0.0))
                    .register(),
            ]);
        let r = Ray::new(Point::zero(), Vector::y_norm());
        w.color_at(r, 5);
    }

    #[test]
    fn the_reflected_color_at_the_maximum_depth() {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let lights = vec![light];
        let s1 = ObjectBuilder::new_sphere()
            .with_material(
                Material::new()
                    .with_color(Color::new(0.8, 1.0, 0.6))
                    .with_diffuse(0.7)
                    .with_specular(0.2),
            )
            .register();
        let s2 = ObjectBuilder::new_sphere().register();
        let shape = ObjectBuilder::new_plane()
            .with_material(Material::new().with_reflective(0.5))
            .with_transform(Transformation::new_transform().translation(0.0, -1.0, 0.0))
            .register();
        let objects = vec![s1, s2, shape];
        let w = World::new().with_lights(lights).with_objects(objects);
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let i = Intersection::new(f64::sqrt(2.0), shape);
        let mut xs = Intersections::new();
        xs.push(i);
        let comps = i.prepare_computations(r, &xs);
        let color = w.reflected_color(&comps, 0);
        assert_eq!(color, Color::black());
    }

    #[test]
    fn the_refracted_color_with_an_opaque_surface() {
        let w = World::default();
        let shape = w.objects().get(0).unwrap();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_norm());
        let i1 = Intersection::new(4.0, *shape);
        let i2 = Intersection::new(6.0, *shape);
        let mut xs = Intersections::new();
        xs.push(i1);
        xs.push(i2);
        let comps = xs[0].prepare_computations(r, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, Color::black());
    }

    #[test]
    fn the_refracted_color_at_the_maximum_recursive_depth() {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let lights = vec![light];
        let m1 = Material::new()
            .with_color(Color::new(0.8, 1.0, 0.6))
            .with_diffuse(0.7)
            .with_specular(0.2);
        let s1 = ObjectBuilder::new_sphere()
            .with_material(m1)
            .with_material(
                Material::new()
                    .with_transparency(1.0)
                    .with_refractive_index(1.5),
            )
            .register();
        let t2 = Transformation::new_transform().scaling(0.5, 0.5, 0.5);
        let s2 = ObjectBuilder::new_sphere().with_transform(t2).register();
        let objects = vec![s1, s2];
        let w = World::new().with_lights(lights).with_objects(objects);
        let shape = w.objects().get(0).unwrap();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_norm());
        let mut xs = Intersections::new();
        xs.push(Intersection::new(4.0, *shape));
        xs.push(Intersection::new(6.0, *shape));
        let comps = xs[0].prepare_computations(r, &xs);
        let c = w.refracted_color(&comps, 0);
        assert_eq!(c, Color::black());
    }

    #[test]
    fn the_refracted_color_under_total_reflection() {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let lights = vec![light];
        let s1 = ObjectBuilder::new_sphere()
            .with_material(
                Material::new()
                    .with_transparency(1.0)
                    .with_refractive_index(1.5),
            )
            .register();
        let t2 = Transformation::new_transform().scaling(0.5, 0.5, 0.5);
        let s2 = ObjectBuilder::new_sphere().with_transform(t2).register();
        let objects = vec![s1, s2];
        let w = World::new().with_lights(lights).with_objects(objects);
        let shape = w.objects().get(0).unwrap();
        let r = Ray::new(Point::new(0.0, 0.0, f64::sqrt(2.0) / 2.0), Vector::y_norm());
        let mut xs = Intersections::new();
        xs.push(Intersection::new(-f64::sqrt(2.0) / 2.0, *shape));
        xs.push(Intersection::new(f64::sqrt(2.0) / 2.0, *shape));
        let comps = xs[1].prepare_computations(r, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, Color::black());
    }

    #[test]
    fn the_refracted_color_with_a_refracted_ray() {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let lights = vec![light];
        let m1 = Material::new()
            .with_color(Color::new(0.8, 1.0, 0.6))
            .with_diffuse(0.7)
            .with_ambient(1.0)
            .with_specular(0.2)
            .with_pattern(Pattern::new_test_pattern());
        let s1 = ObjectBuilder::new_sphere().with_material(m1).register();
        let s2 = ObjectBuilder::new_sphere()
            .with_material(
                Material::new()
                    .with_transparency(1.0)
                    .with_refractive_index(1.5),
            )
            .register();
        let objects = vec![s1.clone(), s2.clone()];
        let w = World::new().with_lights(lights).with_objects(objects);
        let r = Ray::new(Point::new(0.0, 0.0, 0.1), Vector::y_norm());
        let mut xs = Intersections::new();
        xs.push(Intersection::new(-0.9899, s1));
        xs.push(Intersection::new(-0.4899, s2));
        xs.push(Intersection::new(0.4899, s2));
        xs.push(Intersection::new(0.9899, s1));
        let comps = xs[2].prepare_computations(r, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, Color::new(0.0, 0.99888, 0.04722));
    }

    #[test]
    fn shade_hit_with_transparent_material() {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let lights = vec![light];
        let s1 = ObjectBuilder::new_sphere()
            .with_material(
                Material::new()
                    .with_color(Color::new(0.8, 1.0, 0.6))
                    .with_diffuse(0.7)
                    .with_specular(0.2),
            )
            .register();
        let s2 = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().scaling(0.5, 0.5, 0.5))
            .register();
        let floor = ObjectBuilder::new_plane()
            .with_transform(Transformation::new_transform().translation(0.0, -1.0, 0.0))
            .with_material(
                Material::new()
                    .with_transparency(0.5)
                    .with_refractive_index(1.5),
            )
            .register();
        let ball = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().translation(0.0, -3.5, -0.5))
            .with_material(
                Material::new()
                    .with_color(Color::new(1.0, 0.0, 0.0))
                    .with_ambient(0.5),
            )
            .register();
        let objects = vec![s1, s2, floor, ball];
        let w = World::new().with_lights(lights).with_objects(objects);
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let mut xs = Intersections::new();
        xs.push(Intersection::new(f64::sqrt(2.0), floor));
        let comps = xs[0].prepare_computations(r, &xs);
        let c = w.shade_hit(comps, 5);
        assert_eq!(c, Color::new(0.93642, 0.68642, 0.68642));
    }

    #[test]
    fn shade_hit_with_reflective_transparent_material() {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let lights = vec![light];
        let s1 = ObjectBuilder::new_sphere()
            .with_material(
                Material::new()
                    .with_color(Color::new(0.8, 1.0, 0.6))
                    .with_diffuse(0.7)
                    .with_specular(0.2),
            )
            .register();
        let s2 = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().scaling(0.5, 0.5, 0.5))
            .register();
        let floor = ObjectBuilder::new_plane()
            .with_transform(Transformation::new_transform().translation(0.0, -1.0, 0.0))
            .with_material(
                Material::new()
                    .with_reflective(0.5)
                    .with_transparency(0.5)
                    .with_refractive_index(1.5),
            )
            .register();
        let ball = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().translation(0.0, -3.5, -0.5))
            .with_material(
                Material::new()
                    .with_color(Color::new(1.0, 0.0, 0.0))
                    .with_ambient(0.5),
            )
            .register();
        let objects = vec![s1, s2, floor.clone(), ball];
        let w = World::new().with_lights(lights).with_objects(objects);
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let mut xs = Intersections::new();
        xs.push(Intersection::new(f64::sqrt(2.0), floor));
        let comps = xs[0].prepare_computations(r, &xs);
        let c = w.shade_hit(comps, 5);
        assert_eq!(c, Color::new(0.93391, 0.69643, 0.69243));
    }
}
