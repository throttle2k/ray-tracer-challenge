use std::fs::{self};

use colo_rs::colors::Color;
use ray_tracer::{canvas::Canvas, points::Point, ppm::PPM, tuples::Tuple, vectors::Vector};

struct Projectile {
    position: Point,
    velocity: Vector,
}

impl Projectile {
    fn new(position: Point, velocity: Vector) -> Self {
        Self { position, velocity }
    }
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl Environment {
    fn new(gravity: Vector, wind: Vector) -> Self {
        Self { gravity, wind }
    }
}

fn tick(proj: &mut Projectile, env: &Environment) {
    let new_pos = &proj.position + &proj.velocity;
    let new_vel = &proj.velocity + &env.gravity + &env.wind;
    proj.position = new_pos;
    proj.velocity = new_vel;
}

fn main() {
    let start = Point::new(0.0, 1.0, 0.0);
    let velocity = Vector::new(1.0, 1.8, 0.0).normalize() * 11.25;
    let mut p = Projectile::new(start, velocity);

    let gravity = Vector::new(0.0, -0.1, 0.0);
    let wind = Vector::new(-0.01, 0.0, 0.0);
    let e = Environment::new(gravity, wind);

    let mut canvas = Canvas::new(900, 550);

    while p.position.y() > 0.0 {
        let p_draw_x = p.position.x().round() as usize;
        let p_draw_y = if canvas.height() > p.position.y().round() as usize {
            canvas.height() - p.position.y().round() as usize
        } else {
            canvas.height()
        };
        canvas.write_pixel(p_draw_x, p_draw_y, Color::red());
        tick(&mut p, &e);
    }
    let ppm = PPM::from(canvas);
    fs::write("trajectory.ppm", ppm.to_string()).unwrap();
}
