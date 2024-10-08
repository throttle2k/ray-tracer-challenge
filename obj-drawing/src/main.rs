use std::{f64::consts::PI, fs, path::PathBuf};

use colo_rs::colors::Color;
use ray_tracer::{
    camera::Camera,
    lights::PointLight,
    obj_parser::OBJParser,
    ppm::PPM,
    transformations::Transformation,
    tuples::{points::Point, vectors::Vector, Tuple},
    world::World,
};

fn main() {
    let mut path = PathBuf::new();
    path.push("./examples/teapot.obj");
    let g = OBJParser::load_file(&path).unwrap().into_group();
    let light = PointLight::new(Point::new(40.0, 40.0, -40.0), Color::white());
    let w = World::new()
        .with_lights(vec![light])
        .with_objects(vec![g])
        .with_octree();
    let c = Camera::new(320, 240, PI / 3.0).with_transform(
        Transformation::view_transform(
            Point::new(2.0, 3.0, -5.0),
            Point::new(0.0, 2.0, 0.0),
            Vector::y_norm(),
        )
        .translation(0.0, 0.0, -4.0),
    );
    let canvas = c.render(w);
    let ppm = PPM::from(canvas);
    fs::write("./teapot.ppm", ppm.to_string()).unwrap();
}
