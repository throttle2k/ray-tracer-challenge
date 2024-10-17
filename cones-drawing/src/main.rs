use std::{f64::consts::PI, fs};

use colo_rs::colors::Color;
use ray_tracer::{
    camera::Camera,
    lights::PointLight,
    materials::Material,
    patterns::Pattern,
    ppm::PPM,
    shapes::{Cap, ObjectBuilder},
    transformations::Transformation,
    tuples::{points::Point, vectors::Vector, Tuple},
    world::World,
};

fn main() {
    let floor = ObjectBuilder::new_plane()
        .with_transform(Transformation::new_transform().translation(0.0, -1.0, 0.0))
        .with_material(
            Material::new()
                .with_pattern(Pattern::new_solid_pattern(Color::new(1.0, 0.9, 0.9)))
                .with_specular(0.0)
                .with_reflective(0.2)
                .with_receive_shadows(false)
                .with_pattern(
                    Pattern::new_checker_pattern(
                        Pattern::new_striped_pattern(
                            Pattern::new_solid_pattern(Color::new(0.6, 0.3, 0.4)),
                            Pattern::new_solid_pattern(Color::new(0.5, 0.2, 0.2)),
                        )
                        .with_transform(
                            Transformation::new_transform()
                                .scaling(0.3, 0.3, 0.3)
                                .rotation_y(PI / 4.0),
                        ),
                        Pattern::new_striped_pattern(
                            Pattern::new_solid_pattern(Color::new(0.2, 0.2, 0.2)),
                            Pattern::new_solid_pattern(Color::new(0.4, 0.4, 0.4)),
                        )
                        .with_transform(
                            Transformation::new_transform()
                                .scaling(0.3, 0.3, 0.3)
                                .rotation_y(-PI / 4.0),
                        ),
                    )
                    .with_transform(Transformation::new_transform().scaling(1.0, 1.0, 1.0)),
                ),
        )
        .build();

    let cone_1 = ObjectBuilder::new_cone()
        .with_min(-1.0)
        .with_max(0.0)
        .with_cap(Cap::Both)
        .with_transform(Transformation::new_transform().translation(-10.0, 0.0, 0.0))
        .with_material(
            Material::new()
                .with_color(Color::new(1.0, 0.843, 0.0))
                .with_ambient(0.2)
                .with_specular(1.0)
                .with_shininess(1000.0)
                .with_reflective(1.0),
        )
        .build();

    let cone_2 = ObjectBuilder::new_cone()
        .with_min(-1.0)
        .with_max(1.0)
        .with_transform(Transformation::new_transform().translation(-5.0, 0.0, -2.0))
        .with_material(
            Material::new()
                .with_color(Color::new(0.2, 0.0, 0.9))
                .with_specular(1.0)
                .with_shininess(1000.0)
                .with_reflective(1.0),
        )
        .build();

    let cone_3 = ObjectBuilder::new_cone()
        .with_min(-1.0)
        .with_max(2.3)
        .with_transform(Transformation::new_transform().translation(0.0, 0.0, 0.0))
        .with_material(
            Material::new()
                .with_color(Color::red())
                .with_reflective(1.0),
        )
        .build();

    let cone_4 = ObjectBuilder::new_cone()
        .with_min(-2.0)
        .with_max(0.0)
        .with_transform(
            Transformation::new_transform()
                .rotation_x(-PI / 4.0)
                .translation(6.0, 1.9, 1.0),
        )
        .with_material(
            Material::new()
                .with_color(Color::new(0.0, 0.9, 0.1))
                .with_specular(1.0)
                .with_shininess(300.0)
                .with_reflective(1.0),
        )
        .build();

    let cone_5 = ObjectBuilder::new_cone()
        .with_min(-2.0)
        .with_max(0.0)
        .with_transform(
            Transformation::new_transform()
                .rotation_z(-PI / 4.0)
                .rotation_y(-0.3)
                .translation(12.0, 1.9, 0.0),
        )
        .with_material(
            Material::new()
                .with_color(Color::white())
                .with_ambient(0.2)
                .with_diffuse(1.0)
                .with_specular(1.0)
                .with_shininess(300.0)
                .with_reflective(0.9),
        )
        .build();

    let g = ObjectBuilder::new_group()
        .add_child(cone_1)
        .add_child(cone_2)
        .add_child(cone_3)
        .add_child(cone_4)
        .add_child(cone_5)
        .build();

    let light_1 = PointLight::new(
        Point::new(-2.0, 5.0, -10.0),
        Color::new(1.0, 1.0, 1.0) / 2.0,
    );

    let light_2 = PointLight::new(Point::new(5.0, 5.0, -10.0), Color::new(1.0, 1.0, 1.0) / 2.0);

    let light_3 = PointLight::new(Point::new(0.0, 25.0, 100.0), Color::new(0.7, 0.0, 0.0));

    let w = World::new()
        .with_lights(vec![light_1, light_2, light_3])
        .with_objects(vec![floor, g]);

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
    fs::write("cones.ppm", ppm.to_string()).unwrap();
}
