use crate::{
    intersections::Intersections, materials::Material, points::Point, rays::Ray,
    transformations::Transformation, tuples::Tuple, vectors::Vector,
};

#[derive(Debug, Clone)]
pub struct Sphere {
    transform: Transformation,
    material: Material,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            transform: Transformation::new_transform(),
            material: Material::new(),
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

    pub fn intersect<'a>(&'a self, r: &Ray) -> Intersections<'a> {
        if let Some(t) = self.transform.inverse() {
            let ray = r.transform(t);
            ray.intersects(self)
        } else {
            Intersections::new()
        }
    }

    pub fn normal_at(&self, world_point: Point) -> Vector {
        if let Some(m) = self.transform.inverse() {
            let object_point = &m * &world_point;
            let object_normal = object_point - Point::zero();
            let world_normal = &m.transpose() * &object_normal;
            world_normal.normalize()
        } else {
            Vector::zero()
        }
    }
}

impl PartialEq for Sphere {
    fn eq(&self, _other: &Self) -> bool {
        return true;
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::matrix::Matrix;

    use super::*;

    #[test]
    fn sphere_default_trasnsformation() {
        let s = Sphere::new();
        assert_eq!(s.transform, Matrix::identity(4));
    }

    #[test]
    fn changing_sphere_transformation() {
        let t = Transformation::new_transform().translation(2.0, 3.0, 4.0);
        let s = Sphere::new().with_transform(t.clone());
        assert_eq!(s.transform, t);
    }

    #[test]
    fn normal_of_sphere_at_point_on_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(1.0, 0.0, 0.0));
        assert_eq!(n, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_of_sphere_at_point_on_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(0.0, 1.0, 0.0));
        assert_eq!(n, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_of_sphere_at_point_on_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(0.0, 0.0, 1.0));
        assert_eq!(n, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_of_sphere_at_a_non_axial_point() {
        let s = Sphere::new();
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
        let s = Sphere::new();
        let n = s.normal_at(Point::new(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        ));
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let s = Sphere::new()
            .with_transform(Transformation::new_transform().translation(0.0, 1.0, 0.0));
        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));
        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let s = Sphere::new().with_transform(
            Transformation::new_transform()
                .rotation_z(PI / 5.0)
                .scaling(1.0, 0.5, 1.0),
        );
        let n = s.normal_at(Point::new(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0));
        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn sphere_default_material() {
        let s = Sphere::new();
        let m = s.material;
        assert_eq!(m, Material::new());
    }

    #[test]
    fn assigning_a_material_to_a_sphere() {
        let mut s = Sphere::new();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.material = m.clone();
        assert_eq!(s.material, m);
    }
}
