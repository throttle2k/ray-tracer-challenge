use std::fs;

use colo_rs::colors::Color;
use ray_tracer::{
    canvas::Canvas, lights::PointLight, materials::Material, points::Point, ppm::PPM, rays::Ray,
    sphere::Sphere, tuples::Tuple,
};

fn main() {
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 250;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let shape = Sphere::new().with_material(Material::new().with_color(Color::new(1.0, 0.2, 1.0)));
    let light_position = Point::new(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_position, light_color);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;
            let position = Point::new(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(&r);
            if let Some(hit) = xs.hit() {
                let point = r.position(hit.t);
                let normal = hit.object.normal_at(point);
                let eye = -r.direction;
                let color = hit.object.material().lighting(light, point, eye, normal);
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
