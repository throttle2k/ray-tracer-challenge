use std::{f64::consts::PI, fs};

use colo_rs::colors::Color;
use ray_tracer::{
    camera::Camera,
    lights::PointLight,
    materials::Material,
    ppm::PPM,
    shapes::Object,
    transformations::Transformation,
    tuples::{points::Point, vectors::Vector, Tuple},
    world::World,
};

fn main() {
    let floor_m = Material::new()
        .with_color(Color::new(1.0, 0.9, 0.9))
        .with_specular(0.0);
    let floor = Object::new_plane().with_material(floor_m.clone());

    let middle_t = Transformation::new_transform().translation(-0.5, 1.0, 0.5);
    let middle_m = Material::new()
        .with_color(Color::new(0.1, 1.0, 0.5))
        .with_diffuse(0.7)
        .with_specular(0.3);
    let middle = Object::new_sphere()
        .with_material(middle_m)
        .with_transform(middle_t);

    let right_t = Transformation::new_transform()
        .scaling(0.5, 0.5, 0.5)
        .translation(1.5, 0.5, -0.5);
    let right_m = Material::new()
        .with_color(Color::new(0.5, 1.0, 0.1))
        .with_diffuse(0.7)
        .with_specular(0.3);
    let right = Object::new_sphere()
        .with_material(right_m)
        .with_transform(right_t);

    let left_t = Transformation::new_transform()
        .scaling(0.33, 0.33, 0.33)
        .translation(-1.5, 0.33, -0.75);
    let left_m = Material::new()
        .with_color(Color::new(1.0, 0.8, 0.1))
        .with_diffuse(0.7)
        .with_specular(0.3);
    let left = Object::new_sphere()
        .with_material(left_m)
        .with_transform(left_t);

    let light_source = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let camera_t = Transformation::view_transform(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );
    let camera = Camera::new(300, 150, PI / 3.0).with_transform(camera_t);

    let world = World::new()
        .with_objects(vec![floor, middle, right, left])
        .with_lights(vec![light_source]);

    let canvas = camera.render(world);

    let ppm = PPM::from(canvas);
    fs::write("plane.ppm", ppm.to_string()).unwrap();
}
