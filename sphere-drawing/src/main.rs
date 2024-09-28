use std::{f64::consts::PI, fs};

use colo_rs::colors::Color;
use ray_tracer::{
    canvas::Canvas, points::Point, ppm::PPM, rays::Ray, sphere::Sphere,
    transformations::Transformation, tuples::Tuple,
};

fn main() {
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::red();
    let mut shape = Sphere::new();

    let t = Transformation::new_transform()
        .scaling(0.5, 1.0, 1.0)
        .shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    shape.set_transform(t);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;
            let position = Point::new(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(&r);
            if xs.hit().is_some() {
                canvas.write_pixel(x, y, color);
            }
        }
    }

    let ppm = PPM::from(canvas);
    fs::write(
        "/home/throttle/development/rust/ray-tracer-challenge/sphere-drawing/sphere.ppm",
        ppm.to_string(),
    )
    .unwrap();
}
