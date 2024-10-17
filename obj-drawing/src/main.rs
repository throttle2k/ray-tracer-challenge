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
    let mut g = OBJParser::load_file(&path).unwrap().into_group().build();

    g.divide(100);

    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::white());
    let w = World::new().with_lights(vec![light]).with_objects(vec![g]);
    let c = Camera::new(640, 480, PI / 3.0).with_transform(Transformation::view_transform(
        Point::new(0.0, 2.0, -7.0),
        Point::new(0.0, 2.0, 0.0),
        Vector::y_norm(),
    ));
    let canvas = c.render(w);
    let ppm = PPM::from(canvas);
    fs::write("./teapot.ppm", ppm.to_string()).unwrap();
}
