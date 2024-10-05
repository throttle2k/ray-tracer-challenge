use std::{f64::consts::PI, fs};

use colo_rs::colors::Color;
use ray_tracer::{
    camera::Camera,
    lights::PointLight,
    materials::Material,
    ppm::PPM,
    shapes::{cylinder::CylinderCap, Cylinder, ObjectBuilder},
    transformations::Transformation,
    tuples::{points::Point, vectors::Vector, Tuple},
    world::World,
};

fn main() {
    let cyl_len = 4.0;

    let cyl_1 = ObjectBuilder::new_cylinder(
        Cylinder::default()
            .with_minimum(-cyl_len * 1.5)
            .with_maximum(cyl_len * 0.5)
            .with_cap(CylinderCap::Uncapped),
    )
    .with_material(
        Material::new()
            .with_color(Color::new(0.722, 0.451, 0.20))
            .with_specular(1.0)
            .with_shininess(10.0)
            .with_reflective(0.9),
    )
    .register();

    let cyl_2 = ObjectBuilder::new_cylinder(
        Cylinder::default()
            .with_minimum(-cyl_len * 2.0)
            .with_maximum(cyl_len * 4.0)
            .with_cap(CylinderCap::Uncapped),
    )
    .with_transform(
        Transformation::new_transform()
            .rotation_z(PI / 4.0)
            .rotation_x(PI / 2.0)
            .translation(0.0, 0.0, -5.0),
    )
    .with_material(
        Material::new()
            .with_color(Color::new(0.722, 0.451, 0.20))
            .with_specular(1.0)
            .with_shininess(10.0)
            .with_reflective(0.9),
    )
    .register();

    let cyl_3 = ObjectBuilder::new_cylinder(
        Cylinder::default()
            .with_minimum(-cyl_len * 2.0)
            .with_maximum(cyl_len * 3.0)
            .with_cap(CylinderCap::Uncapped),
    )
    .with_transform(
        Transformation::new_transform()
            .rotation_z(-PI / 4.0)
            .translation(4.0, 0.0, 0.0)
            .translation(0.0, 0.0, 5.0),
    )
    .with_material(
        Material::new()
            .with_color(Color::new(0.722, 0.451, 0.20))
            .with_specular(1.0)
            .with_shininess(10.0)
            .with_reflective(0.9),
    )
    .register();

    let cyl_4 = ObjectBuilder::new_cylinder(
        Cylinder::default()
            .with_minimum(-cyl_len * 3.0)
            .with_maximum(cyl_len * 1.5)
            .with_cap(CylinderCap::Uncapped),
    )
    .with_transform(Transformation::new_transform().translation(-10.5, 0.0, 10.0))
    .with_material(
        Material::new()
            .with_color(Color::new(0.7922, 0.8, 0.8078))
            .with_diffuse(0.3)
            .with_specular(0.8)
            .with_shininess(10.0)
            .with_reflective(0.5),
    )
    .register();

    let cyl_5 = ObjectBuilder::new_cylinder(
        Cylinder::default()
            .with_minimum(-cyl_len * 1.0)
            .with_maximum(cyl_len * 1.0)
            .with_cap(CylinderCap::Uncapped),
    )
    .with_transform(
        Transformation::new_transform()
            .rotation_x(-PI / 2.0)
            .translation(3.0, 4.0, -4.0),
    )
    .with_material(
        Material::new()
            .with_color(Color::new(0.7922, 0.8, 0.8078))
            .with_diffuse(0.3)
            .with_specular(0.8)
            .with_shininess(10.0)
            .with_reflective(0.5),
    )
    .register();

    let light = PointLight::new(Point::new(-2.0, 5.0, -10.0), Color::white());

    let w = World::new()
        .with_lights(vec![light])
        .with_objects(vec![cyl_1, cyl_2, cyl_3, cyl_4, cyl_5]);

    let c = Camera::new(1024, 768, PI / 3.0).with_transform(
        Transformation::view_transform(
            Point::new(0.0, 1.5, -5.0),
            Point::new(0.0, 0.5, 0.0),
            Vector::y_norm(),
        )
        .translation(0.0, 0.0, -20.0),
    );
    let canvas = c.render(w);

    let ppm = PPM::from(canvas);
    fs::write("open-cylinders.ppm", ppm.to_string()).unwrap();
}
