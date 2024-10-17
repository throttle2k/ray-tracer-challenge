use std::{f64::consts::PI, fs};

use colo_rs::colors::Color;
use ray_tracer::{
    camera::Camera,
    lights::PointLight,
    materials::Material,
    patterns::Pattern,
    ppm::PPM,
    shapes::ObjectBuilder,
    transformations::Transformation,
    tuples::{points::Point, vectors::Vector, Tuple},
    world::World,
};

fn main() {
    let l1 = PointLight::new(Point::new(-5.0, 5.0, -5.0), Color::white() * 0.6);
    let l2 = PointLight::new(Point::new(5.0, 5.0, -5.0), Color::white() * 0.4);

    let wall_material = Material::new()
        .with_pattern(Pattern::new_solid_pattern(Color::white()))
        .with_specular(0.0);

    let floor = ObjectBuilder::new_plane()
        .with_material(Material::new().with_reflective(0.3).with_pattern(
            Pattern::new_checker_pattern(
                Pattern::new_solid_pattern(Color::white()),
                Pattern::new_solid_pattern(Color::black()),
            ),
        ))
        .with_transform(Transformation::new_transform().rotation_y(PI / 4.0))
        .build();

    let roof = ObjectBuilder::new_plane()
        .with_material(wall_material.clone())
        .with_transform(Transformation::new_transform().translation(0.0, 10.0, 0.0))
        .build();

    let left_wall = ObjectBuilder::new_plane()
        .with_material(wall_material.clone())
        .with_transform(
            Transformation::new_transform()
                .rotation_x(PI / 2.0)
                .rotation_y(-PI / 4.0)
                .translation(0.0, 0.0, 5.0),
        )
        .build();

    let right_wall = ObjectBuilder::new_plane()
        .with_material(wall_material.clone())
        .with_transform(
            Transformation::new_transform()
                .rotation_x(PI / 2.0)
                .rotation_y(PI / 4.0)
                .translation(0.0, 0.0, 5.0),
        )
        .build();

    let s1 = ObjectBuilder::new_sphere()
        .with_material(
            Material::new()
                .with_pattern(
                    Pattern::new_striped_pattern(
                        Pattern::new_solid_pattern(Color::new(1.0, 0.0, 0.0)),
                        Pattern::new_solid_pattern(Color::new(1.0, 1.0, 0.0)),
                    )
                    .with_transform(
                        Transformation::new_transform()
                            .scaling(0.2, 0.2, 0.2)
                            .translation(-1.0, 0.0, 0.0)
                            .rotation_z(-PI / 4.0)
                            .rotation_y(-PI / 4.0),
                    ),
                )
                .with_diffuse(0.9)
                .with_specular(1.0)
                .with_shininess(400.0)
                .with_reflective(0.1),
        )
        .with_transform(
            Transformation::new_transform()
                .scaling(0.5, 0.5, 0.5)
                .translation(1.5, 0.5, 0.5),
        )
        .build();

    let transparent_material = Material::new()
        .with_pattern(Pattern::new_solid_pattern(Color::black()))
        .with_diffuse(0.0)
        .with_specular(0.0)
        .with_ambient(0.0)
        .with_reflective(1.0)
        .with_transparency(1.0)
        .with_refractive_index(1.5);

    let s2 = ObjectBuilder::new_sphere()
        .with_material(transparent_material.clone())
        .with_transform(
            Transformation::new_transform()
                .scaling(0.75, 0.75, 0.75)
                .translation(3.0, 0.75, -3.0),
        )
        .build();

    let s3 = ObjectBuilder::new_sphere()
        .with_material(transparent_material.clone())
        .with_transform(
            Transformation::new_transform()
                .scaling(0.33, 0.33, 0.33)
                .translation(-1.5, 0.33, -0.75),
        )
        .build();

    let s4 = ObjectBuilder::new_sphere()
        .with_material(transparent_material.clone())
        .with_transform(
            Transformation::new_transform()
                .scaling(0.5, 0.5, 0.5)
                .translation(-1.5, 0.5, -3.0),
        )
        .build();

    let s5 = ObjectBuilder::new_sphere()
        .with_material(transparent_material.clone())
        .with_transform(
            Transformation::new_transform()
                .scaling(0.5, 0.5, 0.5)
                .translation(2.0, 0.5, -4.0),
        )
        .build();

    let world = World::new()
        .with_objects(vec![floor, roof, left_wall, right_wall, s1, s2, s3, s4, s5])
        .with_lights(vec![l1, l2]);

    let width = 1024;
    let height = 768;

    let camera =
        Camera::new(width, height, PI / 3.0).with_transform(Transformation::view_transform(
            Point::new(5.0, 1.5, -8.0),
            Point::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        ));

    let canvas = camera.render(world);

    let ppm = PPM::from(canvas);
    fs::write("refractions.ppm", ppm.to_string()).unwrap();
}
