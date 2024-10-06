use crate::{
    lights::PointLight,
    patterns::Pattern,
    shapes::Object,
    tuples::{points::Point, vectors::Vector},
};
use colo_rs::colors::Color;

#[derive(Debug, PartialEq, Clone)]
pub struct Material {
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub pattern: Pattern,
    pub reflective: f64,
    pub transparency: f64,
    pub refractive_index: f64,
    pub cast_shadows: bool,
    pub receive_shadows: bool,
}

impl Material {
    pub fn new() -> Self {
        Self {
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: Pattern::new_solid_pattern(Color::white()),
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            cast_shadows: true,
            receive_shadows: true,
        }
    }

    pub fn with_ambient(mut self, ambient: f64) -> Self {
        self.ambient = ambient;
        self
    }

    pub fn with_diffuse(mut self, diffuse: f64) -> Self {
        self.diffuse = diffuse;
        self
    }

    pub fn with_specular(mut self, specular: f64) -> Self {
        self.specular = specular;
        self
    }

    pub fn with_shininess(mut self, shininess: f64) -> Self {
        self.shininess = shininess;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.pattern = Pattern::new_solid_pattern(color);
        self
    }

    pub fn with_pattern(mut self, pattern: Pattern) -> Self {
        self.pattern = pattern;
        self
    }

    pub fn with_reflective(mut self, reflective: f64) -> Self {
        self.reflective = reflective;
        self
    }

    pub fn with_transparency(mut self, transparency: f64) -> Self {
        self.transparency = transparency;
        self
    }

    pub fn with_refractive_index(mut self, refractive_index: f64) -> Self {
        self.refractive_index = refractive_index;
        self
    }

    pub fn with_cast_shadows(mut self, cast_shadow: bool) -> Self {
        self.cast_shadows = cast_shadow;
        self
    }

    pub fn with_receive_shadows(mut self, receive_shadows: bool) -> Self {
        self.receive_shadows = receive_shadows;
        self
    }

    pub fn lighting(
        &self,
        light: PointLight,
        position: Point,
        eye: Vector,
        normal: Vector,
        in_shadow: bool,
        object: &Object,
    ) -> Color {
        let effective_color = &self.pattern.pattern_at_object(object, position) * &light.intensity;
        let ambient = effective_color * self.ambient;
        let (diffuse, specular) = if in_shadow {
            (Color::black(), Color::black())
        } else {
            let light_vector = (light.position - position).normalize();
            let light_dot_normal = light_vector.dot(normal);
            if light_dot_normal < 0.0 {
                (Color::black(), Color::black())
            } else {
                let diffuse = effective_color * self.diffuse * light_dot_normal;
                //let reflect_vector = (-light_vector).reflect(normal);
                let reflect_vector = -light_vector - normal * 2.0 * -light_vector.dot(normal);
                let reflect_dot_eye = reflect_vector.dot(eye);
                let specular = if reflect_dot_eye <= 0.0 {
                    Color::black()
                } else {
                    let factor = reflect_dot_eye.powf(self.shininess);
                    light.intensity * self.specular * factor
                };
                (diffuse, specular)
            }
        };
        &(&ambient + &diffuse) + &specular
    }
}

#[cfg(test)]
mod tests {
    use crate::{shapes::ObjectBuilder, tuples::Tuple, REGISTRY};

    use super::*;

    #[test]
    fn default_material() {
        let m = Material::new();
        assert_eq!(
            m.pattern,
            Pattern::new_solid_pattern(Color::new(1.0, 1.0, 1.0))
        );
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let m = Material::new();
        let position = Point::zero();
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = false;
        let object = ObjectBuilder::new_test_shape().register();
        let registry = REGISTRY.read().unwrap();
        let object = registry.get_object(object).unwrap();
        let result = m.lighting(light, position, eyev, normalv, in_shadow, object);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45_deg() {
        let m = Material::new();
        let position = Point::zero();
        let eyev = Vector::new(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = false;
        let object = ObjectBuilder::new_test_shape().register();
        let registry = REGISTRY.read().unwrap();
        let object = registry.get_object(object).unwrap();
        let result = m.lighting(light, position, eyev, normalv, in_shadow, object);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_deg() {
        let m = Material::new();
        let position = Point::zero();
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = false;
        let object = ObjectBuilder::new_test_shape().register();
        let registry = REGISTRY.read().unwrap();
        let object = registry.get_object(object).unwrap();
        let result = m.lighting(light, position, eyev, normalv, in_shadow, object);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_reflection() {
        let m = Material::new();
        let position = Point::zero();
        let eyev = Vector::new(0.0, -f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = false;
        let object = ObjectBuilder::new_test_shape().register();
        let registry = REGISTRY.read().unwrap();
        let object = registry.get_object(object).unwrap();
        let result = m.lighting(light, position, eyev, normalv, in_shadow, object);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_eye_behind_surface() {
        let m = Material::new();
        let position = Point::zero();
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = false;
        let object = ObjectBuilder::new_test_shape().register();
        let registry = REGISTRY.read().unwrap();
        let object = registry.get_object(object).unwrap();
        let result = m.lighting(light, position, eyev, normalv, in_shadow, object);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let m = Material::new();
        let position = Point::zero();
        let eye_v = Vector::new(0.0, 0.0, -1.0);
        let normal_v = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = true;
        let object = ObjectBuilder::new_test_shape().register();
        let registry = REGISTRY.read().unwrap();
        let object = registry.get_object(object).unwrap();
        let result = m.lighting(light, position, eye_v, normal_v, in_shadow, object);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_a_pattern_applied() {
        let m = Material::new()
            .with_pattern(Pattern::new_striped_pattern(
                Pattern::new_solid_pattern(Color::white()),
                Pattern::new_solid_pattern(Color::black()),
            ))
            .with_ambient(1.0)
            .with_diffuse(0.0)
            .with_specular(0.0);
        let eye_v = Vector::new(0.0, 0.0, -1.0);
        let normal_v = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::white());
        let object1 = ObjectBuilder::new_test_shape().register();
        let object2 = ObjectBuilder::new_test_shape().register();
        let registry = REGISTRY.read().unwrap();
        let object1 = registry.get_object(object1).unwrap();
        let object2 = registry.get_object(object2).unwrap();

        let c1 = m.lighting(
            light,
            Point::new(0.9, 0.0, 0.0),
            eye_v,
            normal_v,
            false,
            object1,
        );
        let c2 = m.lighting(
            light,
            Point::new(1.1, 0.0, 0.0),
            eye_v,
            normal_v,
            false,
            object2,
        );
        assert_eq!(c1, Color::white());
        assert_eq!(c2, Color::black());
    }

    #[test]
    fn reflective_for_default_material() {
        let m = Material::new();
        assert_eq!(m.reflective, 0.0);
    }

    #[test]
    fn transparency_and_refractive_index_for_default_material() {
        let m = Material::new();
        assert_eq!(m.transparency, 0.0);
        assert_eq!(m.refractive_index, 1.0);
    }
}
