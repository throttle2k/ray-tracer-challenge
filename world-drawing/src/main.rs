use std::{f64::consts::PI, fs};

use colo_rs::colors::Color;
use ray_tracer::{
    camera::Camera,
    lights::PointLight,
    materials::Material,
    ppm::PPM,
    shapes::ObjectBuilder,
    transformations::Transformation,
    tuples::{points::Point, vectors::Vector, Tuple},
    world::World,
};

fn main() {
    let floor_t = Transformation::new_transform().scaling(10.0, 0.01, 10.0);
    let floor_m = Material::new()
        .with_color(Color::new(1.0, 0.9, 0.9))
        .with_specular(0.0);
    let floor = ObjectBuilder::new_sphere()
        .with_transform(floor_t)
        .with_material(floor_m.clone())
        .register();

    let left_wall_t = Transformation::new_transform()
        .scaling(10.0, 0.01, 10.0)
        .rotation_x(PI / 2.0)
        .rotation_y(-PI / 4.0)
        .translation(0.0, 0.0, 5.0);
    let left_wall_m = floor_m.clone();
    let left_wall = ObjectBuilder::new_sphere()
        .with_transform(left_wall_t)
        .with_material(left_wall_m)
        .register();

    let right_wall_t = Transformation::new_transform()
        .scaling(10.0, 0.01, 10.0)
        .rotation_x(PI / 2.0)
        .rotation_y(PI / 4.0)
        .translation(0.0, 0.0, 5.0);
    let right_wall_m = floor_m.clone();
    let right_wall = ObjectBuilder::new_sphere()
        .with_transform(right_wall_t)
        .with_material(right_wall_m)
        .register();

    let middle_t = Transformation::new_transform().translation(-0.5, 1.0, 0.5);
    let middle_m = Material::new()
        .with_color(Color::new(0.1, 1.0, 0.5))
        .with_diffuse(0.7)
        .with_specular(0.3);
    let middle = ObjectBuilder::new_sphere()
        .with_material(middle_m)
        .with_transform(middle_t)
        .register();

    let right_t = Transformation::new_transform()
        .scaling(0.5, 0.5, 0.5)
        .translation(1.5, 0.5, -0.5);
    let right_m = Material::new()
        .with_color(Color::new(0.5, 1.0, 0.1))
        .with_diffuse(0.7)
        .with_specular(0.3);
    let right = ObjectBuilder::new_sphere()
        .with_material(right_m)
        .with_transform(right_t)
        .register();

    let left_t = Transformation::new_transform()
        .scaling(0.33, 0.33, 0.33)
        .translation(-1.5, 0.33, -0.75);
    let left_m = Material::new()
        .with_color(Color::new(1.0, 0.8, 0.1))
        .with_diffuse(0.7)
        .with_specular(0.3);
    let left = ObjectBuilder::new_sphere()
        .with_material(left_m)
        .with_transform(left_t)
        .register();

    let light_source = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let camera_t = Transformation::view_transform(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );
    let camera = Camera::new(500, 250, PI / 3.0).with_transform(camera_t);

    let world = World::new()
        .with_objects(vec![floor, left_wall, right_wall, middle, right, left])
        .with_lights(vec![light_source]);

    let canvas = camera.render(world);

    let ppm = PPM::from(canvas);
    fs::write("world.ppm", ppm.to_string()).unwrap();
}
