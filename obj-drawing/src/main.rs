use std::{f64::consts::PI, fs, path::PathBuf};

use colo_rs::colors::Color;
use ray_tracer::{
    camera::Camera,
    lights::PointLight,
    materials::Material,
    obj_parser::OBJParser,
    patterns::Pattern,
    ppm::PPM,
    shapes::ObjectBuilder,
    transformations::Transformation,
    tuples::{points::Point, vectors::Vector, Tuple},
    world::World,
};

fn main() {
    let mut path = PathBuf::new();
    path.push("./examples/teapot_new.obj");
    let mut g = OBJParser::load_file(&path)
        .unwrap()
        .into_group()
        .with_transform(
            Transformation::new_transform()
                .scaling(0.1, 0.1, 0.1)
                .rotation_x(-PI / 2.0),
        )
        .with_material(
            Material::new()
                .with_ambient(0.0)
                .with_diffuse(0.4)
                .with_specular(0.9)
                .with_shininess(300.0)
                .with_reflective(0.9)
                .with_transparency(0.9)
                .with_refractive_index(1.5)
                .with_pattern(Pattern::new_solid_pattern(Color::new(0.0, 0.0, 0.2))),
        )
        .build();

    g.divide(100);

    let room = ObjectBuilder::new_cube()
        .with_transform(
            Transformation::new_transform()
                .scaling(10.0, 10.0, 10.0)
                .translation(0.0, 10.0, 0.0),
        )
        .with_material(
            Material::new().with_pattern(
                Pattern::new_checker_pattern(
                    Pattern::new_solid_pattern(Color::new(0.9, 0.9, 0.9)),
                    Pattern::new_solid_pattern(Color::new(0.7, 0.7, 0.7)),
                )
                .with_transform(Transformation::new_transform().scaling(0.1, 0.1, 0.1)),
            ),
        )
        .build();

    let light = PointLight::new(Point::new(-5.0, 5.0, -5.0), Color::white());
    let w = World::new()
        .with_lights(vec![light])
        .with_objects(vec![g, room]);
    let c = Camera::new(1000, 700, PI / 3.0).with_transform(Transformation::view_transform(
        Point::new(0.0, 3.0, -3.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::y_norm(),
    ));
    let canvas = c.render(w);
    let ppm = PPM::from(canvas);
    fs::write("./teapot.ppm", ppm.to_string()).unwrap();
}
