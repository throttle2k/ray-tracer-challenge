use crate::{
    rays::Ray,
    tuples::{points::Point, vectors::Vector, Tuple},
};

pub struct Sphere {}

impl Sphere {
    pub fn normal_at(object_point: Point) -> Vector {
        object_point - Point::zero()
    }

    pub fn intersect(ray: Ray) -> Vec<f64> {
        let sphere_to_ray = ray.origin - Point::new(0.0, 0.0, 0.0);
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        let mut result = Vec::new();
        if discriminant < 0.0 {
            result
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            result.push(t1);
            result.push(t2);
            result
        }
    }
}
