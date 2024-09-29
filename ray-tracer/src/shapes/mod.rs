use crate::{
    intersections::Intersections,
    materials::Material,
    rays::Ray,
    sphere::Sphere,
    transformations::Transformation,
    tuples::{points::Point, vectors::Vector, Tuple},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Sphere,
    TestShape,
}

impl Shape {
    fn normal_at(&self, object_point: Point) -> Vector {
        match self {
            Shape::Sphere => Sphere::normal_at(object_point),
            Shape::TestShape => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    transform: Transformation,
    material: Material,
    shape: Shape,
}

impl Object {
    pub fn new_sphere() -> Self {
        Self {
            transform: Transformation::new_transform(),
            material: Material::new(),
            shape: Shape::Sphere,
        }
    }

    pub fn new_test_shape() -> Self {
        Self {
            transform: Transformation::new_transform(),
            material: Material::new(),
            shape: Shape::TestShape,
        }
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
            let object_point = &m * &world_point;
            let object_normal = self.shape.normal_at(object_point);
            let world_normal = &m.transpose() * &object_normal;
            world_normal.normalize()
        } else {
            Vector::zero()
        }
    }

    pub fn intersect<'a>(&'a self, r: &Ray) -> Intersections<'a> {
        if let Some(t) = self.transform.inverse() {
            let ray = r.transform(t);
            ray.intersects(self)
        } else {
            Intersections::new()
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.shape == other.shape
    }
}

#[cfg(test)]
mod tests {

    use std::f64::consts::PI;

    use crate::matrix::Matrix;

    use super::*;

    #[test]
    fn object_default_transformation() {
        let s = Object::new_test_shape();
        assert_eq!(s.transform, Matrix::identity(4));
    }

    #[test]
    fn changing_object_transformation() {
        let t = Transformation::new_transform().translation(2.0, 3.0, 4.0);
        let s = Object::new_test_shape().with_transform(t.clone());
        assert_eq!(s.transform, t);
    }

    #[test]
    fn sphere_default_material() {
        let s = Object::new_test_shape();
        let m = s.material;
        assert_eq!(m, Material::new());
    }

    #[test]
    fn assigning_a_material_to_a_sphere() {
        let mut s = Object::new_test_shape();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.material = m.clone();
        assert_eq!(s.material, m);
    }
    #[test]
    fn normal_of_sphere_at_point_on_x_axis() {
        let s = Object::new_sphere();
        let n = s.normal_at(Point::new(1.0, 0.0, 0.0));
        assert_eq!(n, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_of_sphere_at_point_on_y_axis() {
        let s = Object::new_sphere();
        let n = s.normal_at(Point::new(0.0, 1.0, 0.0));
        assert_eq!(n, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_of_sphere_at_point_on_z_axis() {
        let s = Object::new_sphere();
        let n = s.normal_at(Point::new(0.0, 0.0, 1.0));
        assert_eq!(n, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_of_sphere_at_a_non_axial_point() {
        let s = Object::new_sphere();
        let n = s.normal_at(Point::new(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        ));
        assert_eq!(
            n,
            Vector::new(
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
            )
        );
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Object::new_sphere();
        let n = s.normal_at(Point::new(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        ));
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let s = Object::new_sphere()
            .with_transform(Transformation::new_transform().translation(0.0, 1.0, 0.0));
        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));
        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let s = Object::new_sphere().with_transform(
            Transformation::new_transform()
                .rotation_z(PI / 5.0)
                .scaling(1.0, 0.5, 1.0),
        );
        let n = s.normal_at(Point::new(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0));
        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }
}
