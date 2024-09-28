use std::{f64::consts::PI, fs, usize};

use colo_rs::colors::Color;
use ray_tracer::{
    canvas::Canvas, points::Point, ppm::PPM, transformations::Transformation, tuples::Tuple,
};

fn main() {
    let mut canvas = Canvas::new(300, 300);
    let twelve = Point::new(0.0, 100.0, 0.0);
    let rad = PI / 6.0;
    for i in 0..12 {
        let t = Transformation::new_transform()
            .rotation_z(rad * i as f64)
            .translation(150.0, 150.0, 0.0);
        let h = &t * &twelve;
        canvas.write_pixel(h.x().round() as usize, h.y().round() as usize, Color::red());
    }
    let ppm = PPM::from(canvas);
    fs::write(
        "/home/throttle/development/rust/ray-tracer-challenge/analog-clock/clock.ppm",
        ppm.to_string(),
    )
    .unwrap();
}
