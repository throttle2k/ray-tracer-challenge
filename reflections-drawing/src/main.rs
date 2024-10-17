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
    let floor = ObjectBuilder::new_plane()
        .with_material(
            Material::new()
                .with_pattern(Pattern::new_solid_pattern(Color::new(1.0, 0.9, 0.9)))
                .with_specular(0.0)
                .with_reflective(0.08)
                .with_pattern(Pattern::new_blending_pattern(
                    Pattern::new_striped_pattern(
                        Pattern::new_solid_pattern(Color::black()),
                        Pattern::new_solid_pattern(Color::white()),
                    ),
                    Pattern::new_striped_pattern(
                        Pattern::new_solid_pattern(Color::white()),
                        Pattern::new_solid_pattern(Color::black()),
                    )
                    .with_transform(Transformation::new_transform().rotation_y(PI / 2.0)),
                )),
        )
        .build();

    let wall1 = ObjectBuilder::new_plane()
        .with_material(
            Material::new()
                .with_specular(0.0)
                .with_pattern(Pattern::new_blending_pattern(
                    Pattern::new_striped_pattern(
                        Pattern::new_solid_pattern(Color::black()),
                        Pattern::new_solid_pattern(Color::white()),
                    )
                    .with_transform(Transformation::new_transform().scaling(0.5, 0.5, 0.5)),
                    Pattern::new_striped_pattern(
                        Pattern::new_solid_pattern(Color::white()),
                        Pattern::new_solid_pattern(Color::black()),
                    )
                    .with_transform(
                        Transformation::new_transform()
                            .scaling(0.5, 0.5, 0.5)
                            .rotation_y(PI / 2.0),
                    ),
                )),
        )
        .with_transform(
            Transformation::new_transform()
                .rotation_x(PI / 2.0)
                .translation(0.0, 0.0, 4.0),
        )
        .build();

    let wall2 = ObjectBuilder::new_plane()
        .with_material(
            Material::new()
                .with_specular(0.0)
                .with_pattern(Pattern::new_blending_pattern(
                    Pattern::new_striped_pattern(
                        Pattern::new_solid_pattern(Color::black()),
                        Pattern::new_solid_pattern(Color::white()),
                    )
                    .with_transform(Transformation::new_transform().scaling(0.5, 0.5, 0.5)),
                    Pattern::new_striped_pattern(
                        Pattern::new_solid_pattern(Color::white()),
                        Pattern::new_solid_pattern(Color::black()),
                    )
                    .with_transform(
                        Transformation::new_transform()
                            .scaling(0.5, 0.5, 0.5)
                            .rotation_y(PI / 2.0),
                    ),
                )),
        )
        .with_transform(
            Transformation::new_transform()
                .rotation_z(PI / 2.0)
                .translation(-4.0, 0.0, 0.0),
        )
        .build();

    let middle = ObjectBuilder::new_sphere()
        .with_material(
            Material::new()
                .with_pattern(Pattern::new_solid_pattern(Color::white()))
                .with_diffuse(0.7)
                .with_specular(0.3)
                .with_reflective(0.9)
                .with_transparency(0.9)
                .with_refractive_index(1.9),
        )
        .with_transform(
            Transformation::new_transform()
                .translation(1.5, 1.0, -1.0)
                .scaling(2.0, 2.0, 2.0),
        )
        .build();

    let center = ObjectBuilder::new_sphere()
        .with_material(
            Material::new()
                .with_pattern(Pattern::new_solid_pattern(Color::new(0.8, 0.1, 0.1)))
                .with_diffuse(0.7)
                .with_specular(0.3),
        )
        .with_transform(
            Transformation::new_transform()
                .scaling(1.5, 1.5, 1.5)
                .translation(-2.0, 1.5, 3.0),
        )
        .build();

    let left = ObjectBuilder::new_sphere()
        .with_material(
            Material::new()
                .with_pattern(Pattern::new_solid_pattern(Color::new(0.1, 0.1, 0.9)))
                .with_diffuse(0.7)
                .with_specular(0.3),
        )
        .with_transform(
            Transformation::new_transform()
                .scaling(0.9, 0.9, 0.9)
                .translation(-2.0, 0.9, -1.5),
        )
        .build();

    let right = ObjectBuilder::new_sphere()
        .with_material(
            Material::new()
                .with_pattern(Pattern::new_solid_pattern(Color::new(0.5, 1.0, 0.1)))
                .with_diffuse(0.7)
                .with_specular(0.3),
        )
        .with_transform(
            Transformation::new_transform()
                .scaling(0.5, 0.5, 0.5)
                .translation(1.0, 0.5, 3.0),
        )
        .build();

    let light_source = PointLight::new(Point::new(12.0, 20.0, -22.0), Color::new(0.8, 0.8, 0.8));

    let camera_t = Transformation::view_transform(
        Point::new(8.0, 2.4, -6.0),
        Point::new(-2.0, 2.4, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );
    let camera = Camera::new(500, 500, PI / 3.0).with_transform(camera_t);

    let world = World::new()
        .with_objects(vec![floor, wall1, wall2, left, center, right, middle])
        .with_lights(vec![light_source]);

    let canvas = camera.render(world);

    let ppm = PPM::from(canvas);
    fs::write("reflection.ppm", ppm.to_string()).unwrap();
}
