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

fn cube_grid(
    origin: Point,
    material: Material,
    size: f64,
    nx: i32,
    ny: i32,
    nz: i32,
    sep: f64,
    rotation: f64,
) -> Vec<usize> {
    let mut objects: Vec<usize> = Vec::new();
    for x in 0..nx {
        for y in 0..ny {
            for z in 0..nz {
                let cube = ObjectBuilder::new_cube()
                    .with_transform(
                        Transformation::new_transform()
                            .scaling(size, size, size)
                            .translation(origin.x(), origin.y(), origin.z())
                            .translation(x as f64 * sep, y as f64 * sep, z as f64 * sep)
                            .rotation_y(rotation),
                    )
                    .with_material(material.clone())
                    .register();
                objects.push(cube);
            }
        }
    }
    objects
}

fn main() {
    let room = ObjectBuilder::new_cube()
        .with_transform(Transformation::new_transform().scaling(200.0, 200.0, 200.0))
        .with_material(
            Material::new()
                .with_color(Color::new(0.8, 0.8, 1.0))
                .with_diffuse(0.3)
                .with_ambient(0.2)
                .with_specular(0.0)
                .with_shininess(1.0)
                .with_cast_shadows(false),
        )
        .register();

    let sep = 3.0;
    let rotation = PI / 6.0;

    let green_material = Material::new()
        .with_color(Color::new(0.0, 0.8, 0.1))
        .with_specular(0.4)
        .with_reflective(0.7);

    let mut green_cubes = cube_grid(
        Point::new(4.0, 0.0, 5.0),
        green_material,
        1.3,
        3,
        3,
        3,
        sep * 1.3,
        rotation,
    );

    let ball = ObjectBuilder::new_sphere()
        .with_transform(
            Transformation::new_transform()
                .scaling(18.0, 18.0, 18.0)
                .translation(0.0, 0.0, -3.0)
                .translation(-24.0, 0.0, 0.0)
                .rotation_y(rotation),
        )
        .with_material(
            Material::new()
                .with_color(Color::white())
                .with_diffuse(0.1)
                .with_specular(1.0)
                .with_shininess(1000.0)
                .with_reflective(0.5),
        )
        .register();

    let red_material = Material::new()
        .with_color(Color::new(0.9, 0.0, 0.1))
        .with_diffuse(0.7)
        .with_reflective(0.7);

    let mut red_cubes = cube_grid(
        Point::new(4.0, -12.0, -10.0),
        red_material,
        1.0,
        4,
        4,
        4,
        sep,
        rotation,
    );

    let blue_cube = ObjectBuilder::new_cube()
        .with_material(
            Material::new()
                .with_color(Color::new(0.0, 0.1, 0.8))
                .with_reflective(0.7),
        )
        .with_transform(
            Transformation::new_transform()
                .scaling(2.0, 2.0, 2.0)
                .rotation_y(PI / 6.0)
                .rotation_x(PI / 4.0)
                .translation(0.0, 4.0, -5.0),
        )
        .register();

    let light = PointLight::new(Point::new(-2.0, 10.0, -10.0), Color::white());

    let mut objects: Vec<usize> = vec![room];
    objects.append(&mut green_cubes);
    objects.push(ball);
    objects.append(&mut red_cubes);
    objects.push(blue_cube);

    let w = World::new().with_lights(vec![light]).with_objects(objects);

    let c = Camera::new(640, 480, PI / 3.0).with_transform(
        Transformation::view_transform(
            Point::new(0.0, 1.5, -5.0),
            Point::new(0.0, 0.5, 0.0),
            Vector::y_norm(),
        )
        .translation(0.0, 0.0, -20.0),
    );
    let canvas = c.render(w);

    let ppm = PPM::from(canvas);
    fs::write("cubes.ppm", ppm.to_string()).unwrap();
}
