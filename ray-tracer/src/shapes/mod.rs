mod cone;
pub mod cube;
pub mod cylinder;
mod group;
mod plane;
mod sphere;
mod test_shape;

pub use cone::{Cone, ConeCap};
use cube::Cube;
pub use cylinder::Cylinder;
use group::Group;
use plane::Plane;
use sphere::Sphere;
use test_shape::TestShape;

use crate::bounds::Bounds;
use crate::tuples::{points::Point, vectors::Vector};
use crate::{
    intersections::{Intersection, Intersections},
    materials::Material,
    rays::Ray,
    transformations::Transformation,
    REGISTRY,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Sphere,
    TestShape,
    Plane,
    Cube,
    Cylinder(Cylinder),
    Cone(Cone),
    Group(Group),
}

impl Shape {
    fn normal_at(&self, object_point: Point) -> Vector {
        match self {
            Shape::TestShape => TestShape::normal_at(object_point),
            Shape::Sphere => Sphere::normal_at(object_point),
            Shape::Plane => Plane::normal_at(object_point),
            Shape::Cube => Cube::normal_at(object_point),
            Shape::Cylinder(cyl) => cyl.normal_at(object_point),
            Shape::Cone(cone) => cone.normal_at(object_point),
            Shape::Group(group) => group.normal_at(object_point),
        }
    }

    fn intersect(&self, object_id: usize, ray: Ray) -> Intersections {
        let mut intersections = Intersections::new();
        match self {
            Shape::TestShape => unreachable!(),
            Shape::Sphere => {
                let xs = Sphere::intersect(ray);
                xs.iter()
                    .for_each(|x| intersections.push(Intersection::new(*x, object_id)));
            }
            Shape::Plane => {
                let xs = Plane::intersects(ray);
                xs.iter()
                    .for_each(|x| intersections.push(Intersection::new(*x, object_id)));
            }
            Shape::Cube => {
                let xs = Cube::intersects(ray);
                xs.iter()
                    .for_each(|x| intersections.push(Intersection::new(*x, object_id)));
            }
            Shape::Cylinder(cyl) => {
                let xs = cyl.intersects(ray);
                xs.iter()
                    .for_each(|x| intersections.push(Intersection::new(*x, object_id)));
            }
            Shape::Cone(cone) => {
                let xs = cone.intersects(ray);
                xs.iter()
                    .for_each(|x| intersections.push(Intersection::new(*x, object_id)));
            }
            Shape::Group(group) => {
                let xs = group.intersects(ray);
                xs.iter().for_each(|x| intersections.push(*x));
            }
        }
        intersections
    }

    fn bounds(&self) -> Bounds {
        match self {
            Shape::Sphere => Sphere::bounds(),
            Shape::TestShape => TestShape::bounds(),
            Shape::Plane => Plane::bounds(),
            Shape::Cube => Cube::bounds(),
            Shape::Cylinder(cyl) => cyl.bounds(),
            Shape::Cone(cone) => cone.bounds(),
            Shape::Group(group) => group.bounds(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub id: usize,
    transform: Transformation,
    material: Material,
    shape: Shape,
    parent: Option<usize>,
    bounds: Bounds,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectBuilder {
    id: usize,
    transform: Transformation,
    material: Material,
    shape: Shape,
    parent: Option<usize>,
}

impl ObjectBuilder {
    fn new(shape: Shape) -> Self {
        let mut registry = REGISTRY.write().unwrap();
        let id = registry.next_object_id();
        Self {
            id,
            transform: Transformation::new_transform(),
            material: Material::new(),
            shape: shape.clone(),
            parent: None,
        }
    }

    pub fn new_sphere() -> Self {
        ObjectBuilder::new(Shape::Sphere)
    }

    pub fn new_glass_sphere() -> Self {
        let sphere = Self::new_sphere().with_material(
            Material::new()
                .with_transparency(1.0)
                .with_refractive_index(1.5),
        );
        sphere
    }

    pub fn new_test_shape() -> Self {
        Self::new(Shape::TestShape)
    }

    pub fn new_plane() -> Self {
        Self::new(Shape::Plane)
    }

    pub fn new_cube() -> Self {
        Self::new(Shape::Cube)
    }

    pub fn new_cylinder(cyl: Cylinder) -> Self {
        Self::new(Shape::Cylinder(cyl))
    }

    pub fn new_cone(cone: Cone) -> Self {
        Self::new(Shape::Cone(cone))
    }

    pub fn new_group() -> Self {
        let group = Group::new();
        Self::new(Shape::Group(group))
    }

    pub fn with_transform(mut self, transform: Transformation) -> Self {
        self.transform = transform;
        self
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }

    pub fn add_child(mut self, child_id: usize) -> Self {
        match &mut self.shape {
            Shape::Group(group) => {
                {
                    let mut registry = REGISTRY.write().unwrap();
                    let child = registry.get_object_mut(child_id).unwrap();
                    child.parent = Some(self.id);
                }
                group.add_child(child_id);
            }
            _ => panic!("Cannot add child to object"),
        }
        self
    }

    pub fn register(&self) -> usize {
        let bounds = self.shape.bounds().transform(&self.transform);
        let obj = Object {
            id: self.id,
            transform: self.transform.clone(),
            material: self.material.clone(),
            shape: self.shape.clone(),
            parent: self.parent,
            bounds,
        };
        let mut registry = REGISTRY.write().unwrap();
        registry.add_object(obj);
        self.id
    }
}

impl Object {
    pub fn normal_at(&self, world_point: Point) -> Vector {
        let local_point = self.world_to_object(world_point);
        let local_normal = self.shape.normal_at(local_point);
        self.normal_to_world(local_normal)
    }

    pub fn intersects(&self, r: &Ray) -> Intersections {
        let ray = r.transform(self.transform.inverse().unwrap());
        self.shape.intersect(self.id, ray)
    }

    pub fn to_object_space(&self, world_point: &Point) -> Option<Point> {
        if let Some(t) = self.transform.inverse() {
            Some(t * world_point)
        } else {
            None
        }
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    pub fn group(&self) -> &Group {
        match &self.shape {
            Shape::Group(group) => &group,
            _ => panic!("Cannot call group on object"),
        }
    }

    pub fn set_parent(&mut self, parent_id: usize) {
        self.parent = Some(parent_id);
    }

    pub fn world_to_object(&self, point: Point) -> Point {
        let point = if let Some(parent) = self.parent {
            let registry = REGISTRY.read().unwrap();
            let parent = registry.get_object(parent).unwrap();
            parent.world_to_object(point)
        } else {
            point
        };
        self.transform.inverse().unwrap() * &point
    }

    pub fn normal_to_world(&self, normal: Vector) -> Vector {
        let mut normal = self.transform.inverse_transposed().unwrap() * &normal;
        normal = normal.normalize();

        if let Some(parent) = self.parent {
            let registry = REGISTRY.read().unwrap();
            let parent = registry.get_object(parent).unwrap();
            parent.normal_to_world(normal)
        } else {
            normal
        }
    }

    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    pub fn bounds(&self) -> Bounds {
        self.bounds.clone()
    }
}

#[cfg(test)]
mod tests {

    use std::f64::consts::PI;

    use crate::{matrix::Matrix, tuples::Tuple};

    use super::*;

    #[test]
    fn default_transformation() {
        let s = ObjectBuilder::new_test_shape().register();
        let registry = REGISTRY.read().unwrap();
        let s = registry.get_object(s).unwrap();
        assert_eq!(s.transform.matrix, Matrix::identity(4));
    }

    #[test]
    fn changing_transformation() {
        let t = Transformation::new_transform().translation(2.0, 3.0, 4.0);
        let s = ObjectBuilder::new_test_shape()
            .with_transform(t.clone())
            .register();
        let registry = REGISTRY.read().unwrap();
        let s = registry.get_object(s).unwrap();
        assert_eq!(s.transform, t);
    }

    #[test]
    fn default_material() {
        let s = ObjectBuilder::new_test_shape().register();
        let registry = REGISTRY.read().unwrap();
        let s = registry.get_object(s).unwrap();
        let m = s.material.clone();
        assert_eq!(m, Material::new());
    }

    #[test]
    fn assigning_a_material() {
        let s = ObjectBuilder::new_test_shape().register();
        let mut registry = REGISTRY.write().unwrap();
        let s = registry.get_object_mut(s).unwrap();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.material = m.clone();
        assert_eq!(s.material, m);
    }

    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let s = ObjectBuilder::new_test_shape()
            .with_transform(Transformation::new_transform().translation(0.0, 1.0, 0.0))
            .register();
        let mut registry = REGISTRY.write().unwrap();
        let s = registry.get_object_mut(s).unwrap();
        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));
        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let s = ObjectBuilder::new_test_shape()
            .with_transform(
                Transformation::new_transform()
                    .rotation_z(PI / 5.0)
                    .scaling(1.0, 0.5, 1.0),
            )
            .register();
        let mut registry = REGISTRY.write().unwrap();
        let s = registry.get_object_mut(s).unwrap();
        let n = s.normal_at(Point::new(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0));
        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn creating_a_new_group() {
        let g = ObjectBuilder::new_group().register();
        let registry = REGISTRY.read().unwrap();
        let g = registry.get_object(g).unwrap();
        assert!(g.group().is_empty());
        assert_eq!(g.transform, Transformation::new_transform());
    }

    #[test]
    fn adding_a_child_to_a_group() {
        let s = ObjectBuilder::new_test_shape().register();
        let g = ObjectBuilder::new_group().add_child(s).register();
        let registry = REGISTRY.read().unwrap();
        let g = registry.get_object(g).unwrap();
        assert!(!g.group().is_empty());
        assert!(g.group().contains(&s));
        let s = registry.get_object(s).unwrap();
        assert_eq!(s.parent, Some(g.id));
    }

    #[test]
    fn intersecting_a_ray_with_an_empty_group() {
        let g = ObjectBuilder::new_group().register();
        let registry = REGISTRY.read().unwrap();
        let g = registry.get_object(g).unwrap();
        let r = Ray::new(Point::zero(), Vector::z_norm());
        let xs = g.intersects(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersecting_a_ray_with_an_nonempty_group() {
        let s1 = ObjectBuilder::new_sphere().register();
        let s2 = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().translation(0.0, 0.0, -3.0))
            .register();
        let s3 = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().translation(5.0, 0.0, 0.0))
            .register();
        let g = ObjectBuilder::new_group()
            .add_child(s1)
            .add_child(s2)
            .add_child(s3)
            .register();

        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_norm());
        let registry = REGISTRY.read().unwrap();
        let g = registry.get_object(g).unwrap().clone();
        let xs = g.intersects(&r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].object_id, s2);
        assert_eq!(xs[1].object_id, s2);
        assert_eq!(xs[2].object_id, s1);
        assert_eq!(xs[3].object_id, s1);
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let s = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().translation(5.0, 0.0, 0.0))
            .register();
        let g = ObjectBuilder::new_group()
            .with_transform(Transformation::new_transform().scaling(2.0, 2.0, 2.0))
            .add_child(s)
            .register();
        let registry = REGISTRY.read().unwrap();
        let g = registry.get_object(g).unwrap().clone();
        let r = Ray::new(Point::new(10.0, 0.0, -10.0), Vector::z_norm());
        let xs = g.intersects(&r);
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn converting_a_point_from_world_to_object_space() {
        let s = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().translation(5.0, 0.0, 0.0))
            .register();
        let g2 = ObjectBuilder::new_group()
            .with_transform(Transformation::new_transform().scaling(2.0, 2.0, 2.0))
            .add_child(s)
            .register();
        ObjectBuilder::new_group()
            .with_transform(Transformation::new_transform().rotation_y(PI / 2.0))
            .add_child(g2)
            .register();
        let registry = REGISTRY.read().unwrap();
        let s = registry.get_object(s).unwrap().clone();
        let p = s.world_to_object(Point::new(-2.0, 0.0, -10.0));
        assert_eq!(p, Point::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn converting_a_normal_from_object_to_world_space() {
        let s = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().translation(5.0, 0.0, 0.0))
            .register();
        let g2 = ObjectBuilder::new_group()
            .with_transform(Transformation::new_transform().scaling(1.0, 2.0, 3.0))
            .add_child(s)
            .register();
        ObjectBuilder::new_group()
            .with_transform(Transformation::new_transform().rotation_y(PI / 2.0))
            .add_child(g2)
            .register();
        let registry = REGISTRY.read().unwrap();
        let s = registry.get_object(s).unwrap();
        let n = s.normal_to_world(Vector::new(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        ));
        assert_eq!(n, Vector::new(0.28571, 0.42857, -0.85714));
    }

    #[test]
    fn finding_the_normal_on_a_child_object() {
        let s = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().translation(5.0, 0.0, 0.0))
            .register();
        let g2 = ObjectBuilder::new_group()
            .with_transform(Transformation::new_transform().scaling(1.0, 2.0, 3.0))
            .add_child(s)
            .register();
        ObjectBuilder::new_group()
            .with_transform(Transformation::new_transform().rotation_y(PI / 2.0))
            .add_child(g2)
            .register();

        let registry = REGISTRY.read().unwrap();
        let s = registry.get_object(s).unwrap();
        let n = s.normal_at(Point::new(1.7321, 1.1547, -5.5774));
        assert_eq!(n, Vector::new(0.2857, 0.42854, -0.85716));
    }

    #[test]
    fn ray_intersects_bounds_in_a_non_transformed_shape() {
        let c = ObjectBuilder::new_cube().register();
        let g = ObjectBuilder::new_group().add_child(c).register();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_norm());
        let registry = REGISTRY.read().unwrap();
        let g = registry.get_object(g).unwrap();
        dbg!(&g);
        let xs = g.intersects(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn ray_intersects_bounds_in_a_transformed_shape() {
        let c = ObjectBuilder::new_cube()
            .with_transform(Transformation::new_transform().translation(-5.0, 0.0, 0.0))
            .register();
        let g = ObjectBuilder::new_group().add_child(c).register();
        let r = Ray::new(Point::new(-5.0, 0.0, -5.0), Vector::z_norm());
        let registry = REGISTRY.read().unwrap();
        let g = registry.get_object(g).unwrap();
        dbg!(&g);
        let xs = g.intersects(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn ray_intersects_bounds_in_multiple_transformed_shapes() {
        let c1 = ObjectBuilder::new_cube()
            .with_transform(Transformation::new_transform().translation(-5.0, 0.0, 0.0))
            .register();
        let c2 = ObjectBuilder::new_cube()
            .with_transform(Transformation::new_transform().translation(5.0, 0.0, 0.0))
            .register();
        let g = ObjectBuilder::new_group()
            .add_child(c1)
            .add_child(c2)
            .register();
        let r1 = Ray::new(Point::new(-5.0, 0.0, -5.0), Vector::z_norm());
        let r2 = Ray::new(Point::new(5.0, 0.0, -5.0), Vector::z_norm());
        let r0 = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_norm());
        let registry = REGISTRY.read().unwrap();
        let g = registry.get_object(g).unwrap();
        dbg!(&g);
        let xs1 = g.intersects(&r1);
        let xs2 = g.intersects(&r2);
        let xs0 = g.intersects(&r0);
        assert_eq!(xs1.len(), 2);
        assert_eq!(xs1[0].t, 4.0);
        assert_eq!(xs1[1].t, 6.0);
        assert_eq!(xs2.len(), 2);
        assert_eq!(xs2[0].t, 4.0);
        assert_eq!(xs2[1].t, 6.0);
        assert_eq!(xs0.len(), 0);
    }
}
