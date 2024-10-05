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
    let floor_m = Material::new()
        .with_color(Color::new(1.0, 0.9, 0.9))
        .with_specular(0.0)
        .with_pattern(Pattern::new_perturbed_pattern(
            Pattern::new_blending_pattern(
                Pattern::new_striped_pattern(
                    Pattern::new_solid_pattern(Color::white()),
                    Pattern::new_solid_pattern(Color::black()),
                ),
                Pattern::new_striped_pattern(
                    Pattern::new_solid_pattern(Color::white()),
                    Pattern::new_solid_pattern(Color::black()),
                )
                .with_transform(Transformation::new_transform().rotation_y(-PI / 2.0)),
            ),
        ));
    let floor = ObjectBuilder::new_plane()
        .with_material(floor_m.clone())
        .register();

    let middle_t = Transformation::new_transform()
        .scaling(3.0, 3.0, 3.0)
        .translation(0.0, 3.0, -1.5);
    let middle_m = Material::new()
        .with_color(Color::new(0.1, 1.0, 0.5))
        .with_diffuse(0.7)
        .with_specular(0.3)
        .with_pattern(
            Pattern::new_linear_gradient(Color::blue(), Color::green())
                .with_transform(Transformation::new_transform().rotation_y(PI / 4.0)),
        );

    let middle = ObjectBuilder::new_sphere()
        .with_material(middle_m)
        .with_transform(middle_t)
        .register();

    let right_t = Transformation::new_transform()
        .scaling(1.5, 1.5, 1.5)
        .translation(6.5, 1.5, -3.0);
    let right_m = Material::new()
        .with_color(Color::new(0.5, 1.0, 0.1))
        .with_diffuse(0.7)
        .with_specular(0.3)
        .with_pattern(
            Pattern::new_striped_pattern(
                Pattern::new_linear_gradient(Color::blue(), Color::green())
                    .with_transform(Transformation::new_transform().rotation_y(-PI / 4.0)),
                Pattern::new_solid_pattern(Color::black()),
            )
            .with_transform(
                Transformation::new_transform()
                    .scaling(0.2, 0.2, 0.2)
                    .rotation_z(-PI / 4.0),
            ),
        );
    let right = ObjectBuilder::new_sphere()
        .with_material(right_m)
        .with_transform(right_t)
        .register();

    let left_t = Transformation::new_transform()
        .scaling(2.0, 1.0, 2.0)
        .translation(-5.5, 1.0, -1.75);
    let left_m = Material::new()
        .with_color(Color::new(1.0, 0.8, 0.1))
        .with_diffuse(0.7)
        .with_specular(0.3)
        .with_pattern(
            Pattern::new_checker_pattern(
                Pattern::new_striped_pattern(
                    Pattern::new_solid_pattern(Color::white()),
                    Pattern::new_solid_pattern(Color::blue()),
                )
                .with_transform(
                    Transformation::new_transform()
                        .scaling(0.1, 0.1, 0.1)
                        .rotation_y(PI / 4.0),
                ),
                Pattern::new_striped_pattern(
                    Pattern::new_solid_pattern(Color::red()),
                    Pattern::new_solid_pattern(Color::green()),
                )
                .with_transform(
                    Transformation::new_transform()
                        .scaling(0.1, 0.1, 0.1)
                        .rotation_y(-PI / 4.0),
                ),
            )
            .with_transform(Transformation::new_transform().scaling(1.5, 1.5, 1.5)),
        );
    let left = ObjectBuilder::new_sphere()
        .with_material(left_m)
        .with_transform(left_t)
        .register();

    let wall1_t = Transformation::new_transform()
        .rotation_x(PI / 2.0)
        .translation(0.0, 0.0, 5.0);
    let wall1_m = Material::new().with_color(Color::red()).with_pattern(
        Pattern::new_ring_pattern(
            Pattern::new_striped_pattern(
                Pattern::new_solid_pattern(Color::white()),
                Pattern::new_solid_pattern(Color::green()),
            )
            .with_transform(
                Transformation::new_transform()
                    .scaling(0.2, 1.0, 0.2)
                    .rotation_z(PI / 4.0),
            ),
            Pattern::new_solid_pattern(Color::red()),
        )
        .with_transform(Transformation::new_transform().rotation_y(PI / 4.0)),
    );
    let wall1 = ObjectBuilder::new_plane()
        .with_transform(wall1_t)
        .with_material(wall1_m)
        .register();

    let light_source = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let camera_t = Transformation::view_transform(
        Point::new(0.0, 7.5, -15.0),
        Point::new(0.0, 1.0, 5.0),
        Vector::new(0.0, 1.0, 0.0),
    );
    let camera = Camera::new(500, 250, PI / 3.0).with_transform(camera_t);

    let world = World::new()
        .with_objects(vec![floor, middle, right, left, wall1])
        .with_lights(vec![light_source]);

    let canvas = camera.render(world);

    let ppm = PPM::from(canvas);
    fs::write("pattern.ppm", ppm.to_string()).unwrap();
}
