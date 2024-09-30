use std::sync::Mutex;

use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{
    canvas::Canvas, rays::Ray, transformations::Transformation, tuples::points::Point,
    tuples::Tuple, world::World,
};

pub struct Camera {
    h_size: usize,
    v_size: usize,
    field_of_view: f64,
    transform: Transformation,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    pub fn new(h_size: usize, v_size: usize, field_of_view: f64) -> Self {
        let half_view = f64::tan(field_of_view / 2.0);
        let aspect = h_size as f64 / v_size as f64;
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        let pixel_size = (half_width * 2.0) / h_size as f64;

        Self {
            h_size,
            v_size,
            field_of_view,
            transform: Transformation::new_transform(),
            half_width,
            half_height,
            pixel_size,
        }
    }

    pub fn with_transform(mut self, t: Transformation) -> Self {
        self.transform = t;
        self
    }

    pub fn ray_for_pixel(&self, px: f64, py: f64) -> Ray {
        let x_offset = (px + 0.5) * self.pixel_size;
        let y_offset = (py + 0.5) * self.pixel_size;
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;
        let pixel = self.transform.inverse().unwrap() * &Point::new(world_x, world_y, -1.0);
        let origin = self.transform.inverse().unwrap() * &Point::zero();
        let direction = (pixel - origin).normalize();
        Ray::new(origin, direction)
    }

    pub fn render(&self, w: World) -> Canvas {
        let image = Canvas::new(self.h_size, self.v_size);

        let image_mutex = Mutex::new(image);

        let xs = 0..self.h_size;
        let ys = 0..self.v_size;
        let cross = ys.flat_map(|y| xs.clone().map(move |x| (x, y)));
        cross.par_bridge().for_each(|(x, y)| {
            let ray = self.ray_for_pixel(x as f64, y as f64);
            let color = w.color_at(ray);
            let mut canvas = image_mutex.lock().unwrap();
            canvas.write_pixel(x, y, color);
        });
        let image = image_mutex.lock().unwrap();
        image.clone()
    }

    pub fn field_of_view(&self) -> f64 {
        self.field_of_view
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use approx_eq::ApproxEq;
    use colo_rs::colors::Color;

    use crate::{matrix::Matrix, tuples::vectors::Vector};

    use super::*;

    #[test]
    fn contructing_a_camera() {
        let h_size = 160;
        let v_size = 120;
        let field_of_view = PI / 2.0;
        let c = Camera::new(h_size, v_size, field_of_view);
        assert_eq!(c.h_size, 160);
        assert_eq!(c.v_size, 120);
        assert_eq!(c.field_of_view, PI / 2.0);
        assert_eq!(c.transform.matrix, Matrix::identity(4));
    }

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert!(c.pixel_size.approx_eq(0.01));
    }

    #[test]
    fn pixel_size_for_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);
        assert!(c.pixel_size.approx_eq(0.01));
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100.0, 50.0);
        assert_eq!(r.origin, Point::zero());
        assert_eq!(r.direction, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0.0, 0.0);
        assert_eq!(r.origin, Point::zero());
        assert_eq!(r.direction, Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let t = Transformation::new_transform()
            .translation(0.0, -2.0, 5.0)
            .rotation_y(PI / 4.0);
        let c = Camera::new(201, 101, PI / 2.0).with_transform(t);
        let r = c.ray_for_pixel(100.0, 50.0);
        assert_eq!(r.origin, Point::new(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            Vector::new(f64::sqrt(2.0) / 2.0, 0.0, -f64::sqrt(2.0) / 2.0)
        );
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = World::default();
        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::zero();
        let up = Vector::y_norm();
        let t = Transformation::view_transform(from, to, up);
        let c = Camera::new(11, 11, PI / 2.0).with_transform(t);
        let image = c.render(w);
        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}
