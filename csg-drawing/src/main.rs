use std::{f64::consts::PI, fs};

use colo_rs::colors::Color;
use ray_tracer::{
    camera::Camera,
    lights::PointLight,
    materials::Material,
    patterns::Pattern,
    ppm::PPM,
    shapes::{CSGKind, ObjectBuilder},
    transformations::Transformation,
    tuples::{points::Point, vectors::Vector, Tuple},
    world::World,
};

fn main() {
    let floor = ObjectBuilder::new_plane()
        .with_material(Material::new().with_pattern(Pattern::new_checker_pattern(
            Pattern::new_solid_pattern(Color::black()),
            Pattern::new_solid_pattern(Color::white()),
        )))
        .build();
    let light_1 = PointLight::new(Point::new(0.0, 4.0, 0.0), Color::white());
    let light_2 = PointLight::new(Point::new(5.0, 15.0, -5.0), Color::white());

    let cube = ObjectBuilder::new_cube().build();
    let sphere = ObjectBuilder::new_sphere()
        .with_transform(Transformation::new_transform().scaling(1.4, 1.4, 1.4))
        .build();
    let csg_intersection = ObjectBuilder::new_csg(CSGKind::Intersection, sphere, cube).build();
    let cyl1 = ObjectBuilder::new_cylinder()
        .with_min(-4.0)
        .with_max(4.0)
        .with_transform(Transformation::new_transform().scaling(0.75, 1.0, 0.75))
        .with_material(Material::new().with_pattern(Pattern::new_solid_pattern(Color::red())))
        .build();
    let cyl2 = ObjectBuilder::new_cylinder()
        .with_min(-4.0)
        .with_max(4.0)
        .with_transform(
            Transformation::new_transform()
                .scaling(0.75, 1.0, 0.75)
                .rotation_x(PI / 2.0),
        )
        .with_material(Material::new().with_pattern(Pattern::new_solid_pattern(Color::green())))
        .build();
    let cyl3 = ObjectBuilder::new_cylinder()
        .with_min(-4.0)
        .with_max(4.0)
        .with_transform(
            Transformation::new_transform()
                .scaling(0.75, 1.0, 0.75)
                .rotation_z(PI / 2.0),
        )
        .with_material(Material::new().with_pattern(Pattern::new_solid_pattern(Color::blue())))
        .build();
    let csg_union = ObjectBuilder::new_csg(
        CSGKind::Union,
        cyl1,
        ObjectBuilder::new_csg(CSGKind::Union, cyl2, cyl3).build(),
    )
    .build();

    let csg_difference = ObjectBuilder::new_csg(CSGKind::Difference, csg_intersection, csg_union)
        .with_transform(
            Transformation::new_transform()
                .translation(0.0, 1.0, 0.0)
                .rotation_y(PI / 4.0),
        )
        .build();

    let w = World::new()
        .with_lights(vec![light_1, light_2])
        .with_objects(vec![floor, csg_difference]);
    let c = Camera::new(640, 480, PI / 3.0).with_transform(Transformation::view_transform(
        Point::new(0.0, 3.0, -4.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::y_norm(),
    ));
    let canvas = c.render(w);
    let ppm = PPM::from(canvas);
    fs::write("./csg.ppm", ppm.to_string()).unwrap();
}
