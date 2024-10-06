use std::{f64::consts::PI, fs};

use colo_rs::colors::Color;
use ray_tracer::{
    camera::Camera,
    lights::PointLight,
    ppm::PPM,
    shapes::{Cylinder, ObjectBuilder},
    transformations::Transformation,
    tuples::{points::Point, vectors::Vector, Tuple},
    world::World,
};

fn hex_corner() -> usize {
    ObjectBuilder::new_sphere()
        .with_transform(
            Transformation::new_transform()
                .scaling(0.25, 0.25, 0.25)
                .translation(0.0, 0.0, 1.0),
        )
        .register()
}

fn hex_edge() -> usize {
    ObjectBuilder::new_cylinder(Cylinder::default().with_minimum(0.0).with_maximum(1.0))
        .with_transform(
            Transformation::new_transform()
                .scaling(0.25, 1.0, 0.25)
                .rotation_z(-PI / 2.0)
                .rotation_y(-PI / 6.0)
                .translation(0.0, 0.0, -1.0),
        )
        .register()
}

fn hex_side(rotation: f64) -> usize {
    ObjectBuilder::new_group()
        .add_child(hex_corner())
        .add_child(hex_edge())
        .with_transform(Transformation::new_transform().rotation_y(rotation))
        .register()
}

fn hexagon() -> usize {
    let mut hex = ObjectBuilder::new_group();
    for n in 0..=5 {
        hex = hex.add_child(hex_side(n as f64 * PI / 3.0));
    }
    hex.register()
}

fn main() {
    let light = PointLight::new(Point::new(-2.0, 10.0, -10.0), Color::white());

    let w = World::new()
        .with_lights(vec![light])
        .with_objects(vec![hexagon()]);

    let c = Camera::new(1024, 768, PI / 3.0).with_transform(
        Transformation::view_transform(
            Point::new(0.0, 2.5, -3.0),
            Point::new(0.0, 0.0, 0.0),
            Vector::y_norm(),
        )
        .translation(0.0, 0.0, -7.0),
    );
    let canvas = c.render(w);

    let ppm = PPM::from(canvas);
    fs::write("hexagon.ppm", ppm.to_string()).unwrap();
}
