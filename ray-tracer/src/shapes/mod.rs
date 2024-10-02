mod plane;
mod sphere;
mod test_shape;

use plane::Plane;
use sphere::Sphere;
use test_shape::TestShape;

use crate::{
    intersections::{Intersection, Intersections},
    materials::Material,
    rays::Ray,
    transformations::Transformation,
    tuples::{points::Point, vectors::Vector, Tuple},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Sphere,
    TestShape,
    Plane,
}

impl Shape {
    fn normal_at(&self, object_point: Point) -> Vector {
        match self {
            Shape::TestShape => TestShape::normal_at(object_point),
            Shape::Sphere => Sphere::normal_at(object_point),
            Shape::Plane => Plane::normal_at(object_point),
        }
    }

    fn intersect<'a>(&self, object: &'a Object, ray: Ray) -> Intersections<'a> {
        let mut intersections = Intersections::new();
        match self {
            Shape::TestShape => unreachable!(),
            Shape::Sphere => {
                let xs = Sphere::intersect(ray);
                xs.iter()
                    .for_each(|x| intersections.push(Intersection::new(*x, object)));
            }
            Shape::Plane => {
                let xs = Plane::intersects(ray);
                xs.iter()
                    .for_each(|x| intersections.push(Intersection::new(*x, object)));
            }
        }
        intersections
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    transform: Transformation,
    material: Material,
    shape: Shape,
}

impl Object {
    fn new(shape: Shape) -> Self {
        Self {
            transform: Transformation::new_transform(),
            material: Material::new(),
            shape,
        }
    }

    pub fn new_sphere() -> Self {
        Self::new(Shape::Sphere)
    }

    pub fn new_glass_sphere() -> Self {
        Self::new_sphere().with_material(
            Material::new()
                .with_transparency(1.0)
                .with_refractive_index(1.5),
        )
    }

    pub fn new_test_shape() -> Self {
        Self::new(Shape::TestShape)
    }

    pub fn new_plane() -> Self {
        Self::new(Shape::Plane)
    }

    pub fn with_transform(mut self, transform: Transformation) -> Self {
        self.transform = transform;
        self
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    pub fn normal_at(&self, world_point: Point) -> Vector {
        if let Some(m) = self.transform.inverse() {
            let object_point = m * &world_point;
            let object_normal = self.shape.normal_at(object_point);
            let transform_inverse_transpose = self.transform.inverse_transposed().unwrap();
            let world_normal = transform_inverse_transpose * &object_normal;
            world_normal.normalize()
        } else {
            Vector::zero()
        }
    }

    pub fn intersects<'a>(&'a self, r: &Ray) -> Intersections<'a> {
        let ray = r.transform(self.transform.inverse().unwrap());
        self.shape.intersect(self, ray)
    }

    pub fn to_object_space(&self, world_point: &Point) -> Option<Point> {
        if let Some(t) = self.transform.inverse() {
            Some(t * world_point)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use std::f64::consts::PI;

    use crate::matrix::Matrix;

    use super::*;

    #[test]
    fn default_transformation() {
        let s = Object::new_test_shape();
        assert_eq!(s.transform.matrix, Matrix::identity(4));
    }

    #[test]
    fn changing_transformation() {
        let t = Transformation::new_transform().translation(2.0, 3.0, 4.0);
        let s = Object::new_test_shape().with_transform(t.clone());
        assert_eq!(s.transform, t);
    }

    #[test]
    fn default_material() {
        let s = Object::new_test_shape();
        let m = s.material;
        assert_eq!(m, Material::new());
    }

    #[test]
    fn assigning_a_material() {
        let mut s = Object::new_test_shape();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.material = m.clone();
        assert_eq!(s.material, m);
    }

    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let s = Object::new_test_shape()
            .with_transform(Transformation::new_transform().translation(0.0, 1.0, 0.0));
        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));
        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let s = Object::new_test_shape().with_transform(
            Transformation::new_transform()
                .rotation_z(PI / 5.0)
                .scaling(1.0, 0.5, 1.0),
        );
        let n = s.normal_at(Point::new(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0));
        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }
}
