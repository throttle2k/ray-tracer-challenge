use std::{fs, sync::Mutex};

use colo_rs::colors::Color;
use ray_tracer::{
    canvas::Canvas,
    intersections::Intersection,
    lights::PointLight,
    materials::Material,
    patterns::Pattern,
    ppm::PPM,
    rays::Ray,
    shapes::ObjectBuilder,
    tuples::{points::Point, Tuple},
};
use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() {
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 500;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;
    let canvas_mutex = Mutex::new(Canvas::new(canvas_pixels, canvas_pixels));
    let shape = ObjectBuilder::new_sphere()
        .with_material(
            Material::new().with_pattern(Pattern::new_solid_pattern(Color::new(1.0, 0.2, 1.0))),
        )
        .build();
    let light_position = Point::new(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_position, light_color);

    let xs = 0..canvas_pixels;
    let ys = 0..canvas_pixels;
    let cross = ys.flat_map(|y| xs.clone().map(move |x| (x, y)));
    cross.par_bridge().for_each(|(x, y)| {
        let world_x = -half + pixel_size * x as f64;
        let world_y = half - pixel_size * y as f64;
        let position = Point::new(world_x, world_y, wall_z);
        let r = Ray::new(ray_origin, (position - ray_origin).normalize());
        let xs = shape.intersects(&r);
        if let Some(hit) = xs.hit() {
            let point = r.position(hit.t);
            let object = hit.object;
            let normal = object.normal_at(point, Intersection::new(1.0, &object));
            let eye = -r.direction;
            let color = object
                .material()
                .lighting(light, point, eye, normal, false, &object);
            let mut canvas = canvas_mutex.lock().unwrap();
            canvas.write_pixel(x, y, color);
        }
    });

    let canvas = canvas_mutex.lock().unwrap();
    let ppm = PPM::from(canvas.clone());
    fs::write("sphere.ppm", ppm.to_string()).unwrap();
}
