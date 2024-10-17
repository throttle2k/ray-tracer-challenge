mod cone;
pub mod cube;
pub mod cylinder;
mod group;
mod plane;
mod smooth_triangle;
mod sphere;
mod test_shape;
mod triangle;

use std::marker::PhantomData;

pub use cone::Cone;
use cube::Cube;
pub use cylinder::Cylinder;
use group::Group;
use plane::Plane;
use smooth_triangle::SmoothTriangle;
use sphere::Sphere;
use state::{
    InnerMarker, ShapeMarker, WithCone, WithCube, WithCylinder, WithPlane, WithSmoothTriangle,
    WithSphere, WithTestShape, WithTriangle,
};
pub use state::{WithGroup, WithShape};
use test_shape::TestShape;
use triangle::Triangle;

use crate::bounds::Bounds;
use crate::intersections::Intersection;
use crate::tuples::{points::Point, vectors::Vector};
use crate::{
    intersections::Intersections, materials::Material, rays::Ray, transformations::Transformation,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Sphere(Sphere),
    TestShape(TestShape),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
    Cone(Cone),
    Group(Group),
    Triangle(Triangle),
    SmoothTriangle(SmoothTriangle),
}

impl Shape {
    fn bounds(&self) -> Bounds {
        match self {
            Shape::Sphere(s) => s.bounds(),
            Shape::TestShape(s) => s.bounds(),
            Shape::Plane(s) => s.bounds(),
            Shape::Cube(s) => s.bounds(),
            Shape::Cylinder(s) => s.bounds(),
            Shape::Cone(s) => s.bounds(),
            Shape::Group(s) => s.bounds(),
            Shape::Triangle(s) => s.bounds(),
            Shape::SmoothTriangle(s) => s.bounds(),
        }
    }

    fn normal_at(&self, local_point: Point, hit: Intersection) -> Vector {
        match self {
            Shape::Sphere(s) => s.normal_at(local_point),
            Shape::TestShape(s) => s.normal_at(local_point),
            Shape::Plane(s) => s.normal_at(local_point),
            Shape::Cube(s) => s.normal_at(local_point),
            Shape::Cylinder(s) => s.normal_at(local_point),
            Shape::Cone(s) => s.normal_at(local_point),
            Shape::Group(s) => s.normal_at(local_point),
            Shape::Triangle(s) => s.normal_at(local_point),
            Shape::SmoothTriangle(s) => s.normal_at(local_point, hit),
        }
    }

    fn intersects<'a>(&'a self, object: &'a Object, ray: &Ray) -> Intersections<'a> {
        match self {
            Shape::Sphere(s) => s.intersects(object, ray),
            Shape::TestShape(s) => s.intersects(object, ray),
            Shape::Plane(s) => s.intersects(object, ray),
            Shape::Cube(s) => s.intersects(object, ray),
            Shape::Cylinder(s) => s.intersects(object, ray),
            Shape::Cone(s) => s.intersects(object, ray),
            Shape::Group(s) => s.intersects(object, ray),
            Shape::Triangle(s) => s.intersects(object, ray),
            Shape::SmoothTriangle(s) => s.intersects(object, ray),
        }
    }

    fn group_mut(&mut self) -> Option<&mut Group> {
        match self {
            Shape::Group(s) => Some(s),
            _ => None,
        }
    }

    fn group(&self) -> Option<&Group> {
        match self {
            Shape::Group(s) => Some(s),
            _ => None,
        }
    }

    fn remove_child(&mut self, child: &Object) {
        match self {
            Shape::Group(ref mut s) => s.remove_child(child),
            _ => (),
        };
    }

    fn normal(&self) -> Option<Vector> {
        match self {
            Shape::Triangle(s) => Some(s.normal()),
            _ => None,
        }
    }

    fn divide(&mut self, threshold: usize) {
        match self {
            Shape::Group(g) => g.divide(threshold),
            _ => (),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Cap {
    Uncapped,
    TopCap,
    BottomCap,
    Both,
}

trait WithVertices {
    fn p1(&self) -> Option<Point>;
    fn p2(&self) -> Option<Point>;
    fn p3(&self) -> Option<Point>;
}

impl WithVertices for Shape {
    fn p1(&self) -> Option<Point> {
        match self {
            Shape::Triangle(s) => Some(s.p1()),
            Shape::SmoothTriangle(s) => Some(s.p1()),
            _ => None,
        }
    }

    fn p2(&self) -> Option<Point> {
        match self {
            Shape::Triangle(s) => Some(s.p2()),
            Shape::SmoothTriangle(s) => Some(s.p2()),
            _ => None,
        }
    }

    fn p3(&self) -> Option<Point> {
        match self {
            Shape::Triangle(s) => Some(s.p3()),
            Shape::SmoothTriangle(s) => Some(s.p3()),
            _ => None,
        }
    }
}

trait WithEdges {
    fn e1(&self) -> Option<Vector>;
    fn e2(&self) -> Option<Vector>;
}

impl WithEdges for Shape {
    fn e1(&self) -> Option<Vector> {
        match self {
            Shape::Triangle(s) => Some(s.e1()),
            _ => None,
        }
    }

    fn e2(&self) -> Option<Vector> {
        match self {
            Shape::Triangle(s) => Some(s.e2()),
            _ => None,
        }
    }
}

trait WithVertexNormals {
    fn n1(&self) -> Option<Vector>;
    fn n2(&self) -> Option<Vector>;
    fn n3(&self) -> Option<Vector>;
}

impl WithVertexNormals for Shape {
    fn n1(&self) -> Option<Vector> {
        match self {
            Shape::SmoothTriangle(s) => Some(s.n1()),
            _ => None,
        }
    }

    fn n2(&self) -> Option<Vector> {
        match self {
            Shape::SmoothTriangle(s) => Some(s.n2()),
            _ => None,
        }
    }

    fn n3(&self) -> Option<Vector> {
        match self {
            Shape::SmoothTriangle(s) => Some(s.n3()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    transform: Transformation,
    material: Material,
    shape: Shape,
    bounds: Bounds,
}

mod state {
    pub enum WithShape {}

    pub enum WithSphere {}
    pub enum WithTestShape {}
    pub enum WithPlane {}
    pub enum WithCube {}
    pub enum WithCylinder {}
    pub enum WithCone {}
    pub enum WithGroup {}
    pub enum WithTriangle {}
    pub enum WithSmoothTriangle {}

    pub trait ShapeMarker {}
    impl ShapeMarker for () {}
    impl ShapeMarker for WithShape {}

    pub trait InnerMarker {}
    impl InnerMarker for () {}
    impl InnerMarker for WithSphere {}
    impl InnerMarker for WithTestShape {}
    impl InnerMarker for WithPlane {}
    impl InnerMarker for WithCube {}
    impl InnerMarker for WithCylinder {}
    impl InnerMarker for WithCone {}
    impl InnerMarker for WithGroup {}
    impl InnerMarker for WithTriangle {}
    impl InnerMarker for WithSmoothTriangle {}
}

pub enum Cappable {
    WithCone(WithCone),
    WithCylinder(WithCylinder),
}

impl From<WithCone> for Cappable {
    fn from(with_cone: WithCone) -> Self {
        Self::WithCone(with_cone)
    }
}

impl From<WithCylinder> for Cappable {
    fn from(with_cylinder: WithCylinder) -> Self {
        Self::WithCylinder(with_cylinder)
    }
}

pub enum CanSetVertices {
    WithTriangle(WithTriangle),
    WithSmoothTriangle(WithSmoothTriangle),
}

impl From<WithTriangle> for CanSetVertices {
    fn from(with_triangle: WithTriangle) -> Self {
        Self::WithTriangle(with_triangle)
    }
}

impl From<WithSmoothTriangle> for CanSetVertices {
    fn from(with_smooth_triangle: WithSmoothTriangle) -> Self {
        Self::WithSmoothTriangle(with_smooth_triangle)
    }
}

pub enum CanSetVertexNormals {
    WithSmoothTriangle(WithSmoothTriangle),
}

impl From<WithSmoothTriangle> for CanSetVertexNormals {
    fn from(with_smooth_triangle: WithSmoothTriangle) -> Self {
        Self::WithSmoothTriangle(with_smooth_triangle)
    }
}

pub struct ObjectBuilder<S: ShapeMarker, I: InnerMarker> {
    transform: Transformation,
    material: Material,
    shape: Option<Shape>,
    _shape: PhantomData<S>,
    _inner: PhantomData<I>,
}

impl<S: ShapeMarker, I: InnerMarker> Default for ObjectBuilder<S, I> {
    fn default() -> Self {
        Self {
            transform: Transformation::new_transform(),
            material: Material::new(),
            shape: None,
            _shape: PhantomData,
            _inner: PhantomData,
        }
    }
}

impl<S: ShapeMarker, I: InnerMarker> ObjectBuilder<S, I> {
    pub fn with_transform(mut self, transform: Transformation) -> ObjectBuilder<S, I> {
        self.transform = transform;
        self
    }

    pub fn with_material(mut self, material: Material) -> ObjectBuilder<S, I> {
        self.material = material;
        self
    }
}

impl<I: InnerMarker> ObjectBuilder<WithShape, I> {
    pub fn build(mut self) -> Object {
        match self.shape {
            Some(Shape::Group(ref mut g)) => {
                g.children_mut().iter_mut().for_each(|child| {
                    child.apply_transform(&self.transform);
                });
                self.transform = Transformation::new_transform();
            }
            Some(_) => (),
            None => (),
        };
        let bounds = self
            .shape
            .as_ref()
            .unwrap()
            .bounds()
            .transform(&self.transform);
        Object {
            transform: self.transform.clone(),
            material: self.material.clone(),
            shape: self.shape.unwrap(),
            bounds,
        }
    }
}

impl ObjectBuilder<(), ()> {
    pub fn new_sphere() -> ObjectBuilder<WithShape, WithSphere> {
        ObjectBuilder {
            shape: Some(Shape::Sphere(Sphere::default())),
            _shape: PhantomData,
            _inner: PhantomData,
            ..Default::default()
        }
    }

    pub fn new_glass_sphere() -> ObjectBuilder<WithShape, WithSphere> {
        let sphere = Self::new_sphere().with_material(
            Material::new()
                .with_transparency(1.0)
                .with_refractive_index(1.5),
        );
        sphere
    }

    pub fn new_test_shape() -> ObjectBuilder<WithShape, WithTestShape> {
        ObjectBuilder {
            shape: Some(Shape::TestShape(TestShape::default())),
            _shape: PhantomData,
            _inner: PhantomData,
            ..Default::default()
        }
    }

    pub fn new_plane() -> ObjectBuilder<WithShape, WithPlane> {
        ObjectBuilder {
            shape: Some(Shape::Plane(Plane::default())),
            _shape: PhantomData,
            _inner: PhantomData,
            ..Default::default()
        }
    }

    pub fn new_cube() -> ObjectBuilder<WithShape, WithCube> {
        ObjectBuilder {
            shape: Some(Shape::Cube(Cube::default())),
            _shape: PhantomData,
            _inner: PhantomData,
            ..Default::default()
        }
    }

    pub fn new_cylinder() -> ObjectBuilder<WithShape, WithCylinder> {
        ObjectBuilder {
            shape: Some(Shape::Cylinder(Cylinder::default())),
            _shape: PhantomData,
            _inner: PhantomData,
            ..Default::default()
        }
    }

    pub fn new_cone() -> ObjectBuilder<WithShape, WithCone> {
        ObjectBuilder {
            shape: Some(Shape::Cone(Cone::default())),
            _shape: PhantomData,
            _inner: PhantomData,
            ..Default::default()
        }
    }

    pub fn new_group() -> ObjectBuilder<WithShape, WithGroup> {
        ObjectBuilder {
            shape: Some(Shape::Group(Group::default())),
            _shape: PhantomData,
            _inner: PhantomData,
            ..Default::default()
        }
    }

    pub fn new_triangle() -> ObjectBuilder<WithShape, WithTriangle> {
        ObjectBuilder {
            shape: Some(Shape::Triangle(Triangle::default())),
            _shape: PhantomData,
            _inner: PhantomData,
            ..Default::default()
        }
    }

    pub fn new_smooth_triangle() -> ObjectBuilder<WithShape, WithSmoothTriangle> {
        ObjectBuilder {
            shape: Some(Shape::SmoothTriangle(SmoothTriangle::default())),
            _shape: PhantomData,
            _inner: PhantomData,
            ..Default::default()
        }
    }
}

impl ObjectBuilder<WithShape, WithSphere> {
    pub fn with_shape(self, shape: Sphere) -> ObjectBuilder<WithShape, WithSphere> {
        ObjectBuilder {
            shape: Some(Shape::Sphere(shape)),
            ..self
        }
    }
}

impl<C: InnerMarker + Into<Cappable>> ObjectBuilder<WithShape, C> {
    pub fn with_min(mut self, min: f64) -> Self {
        let mut shape = self.shape.unwrap();
        match shape {
            Shape::Cylinder(ref mut s) => s.with_min(min),
            Shape::Cone(ref mut s) => s.with_min(min),
            _ => unreachable!(),
        };
        self.shape = Some(shape);
        self
    }

    pub fn with_max(mut self, max: f64) -> Self {
        let mut shape = self.shape.unwrap();
        match shape {
            Shape::Cylinder(ref mut s) => s.with_max(max),
            Shape::Cone(ref mut s) => s.with_max(max),
            _ => unreachable!(),
        };
        self.shape = Some(shape);
        self
    }

    pub fn with_cap(mut self, cap: Cap) -> Self {
        let mut shape = self.shape.unwrap();
        match shape {
            Shape::Cylinder(ref mut s) => s.with_cap(cap),
            Shape::Cone(ref mut s) => s.with_cap(cap),
            _ => unreachable!(),
        };
        self.shape = Some(shape);
        self
    }
}

impl ObjectBuilder<WithShape, WithGroup> {
    pub fn add_child(mut self, child: Object) -> Self {
        let mut shape = self.shape.unwrap();
        match shape {
            Shape::Group(ref mut s) => s.add_child(child),
            _ => unreachable!(),
        };
        self.shape = Some(shape);
        self
    }
}

impl<I: InnerMarker + Into<CanSetVertices>> ObjectBuilder<WithShape, I> {
    pub fn set_p1(mut self, p1: Point) -> Self {
        let mut shape = self.shape.unwrap();
        match shape {
            Shape::Triangle(ref mut s) => s.set_p1(p1),
            Shape::SmoothTriangle(ref mut s) => s.set_p1(p1),
            _ => unreachable!(),
        };
        self.shape = Some(shape);
        self
    }

    pub fn set_p2(mut self, p2: Point) -> Self {
        let mut shape = self.shape.unwrap();
        match shape {
            Shape::Triangle(ref mut s) => s.set_p2(p2),
            Shape::SmoothTriangle(ref mut s) => s.set_p2(p2),
            _ => unreachable!(),
        };
        self.shape = Some(shape);
        self
    }

    pub fn set_p3(mut self, p3: Point) -> Self {
        let mut shape = self.shape.unwrap();
        match shape {
            Shape::Triangle(ref mut s) => s.set_p3(p3),
            Shape::SmoothTriangle(ref mut s) => s.set_p3(p3),
            _ => unreachable!(),
        };
        self.shape = Some(shape);
        self
    }
}

impl<I: InnerMarker + Into<CanSetVertexNormals>> ObjectBuilder<WithShape, I> {
    pub fn set_n1(mut self, n1: Vector) -> Self {
        let mut shape = self.shape.unwrap();
        match shape {
            Shape::SmoothTriangle(ref mut s) => s.set_n1(n1),
            _ => unreachable!(),
        };
        self.shape = Some(shape);
        self
    }

    pub fn set_n2(mut self, n2: Vector) -> Self {
        let mut shape = self.shape.unwrap();
        match shape {
            Shape::SmoothTriangle(ref mut s) => s.set_n2(n2),
            _ => unreachable!(),
        };
        self.shape = Some(shape);
        self
    }

    pub fn set_n3(mut self, n3: Vector) -> Self {
        let mut shape = self.shape.unwrap();
        match shape {
            Shape::SmoothTriangle(ref mut s) => s.set_n3(n3),
            _ => unreachable!(),
        };
        self.shape = Some(shape);
        self
    }
}

impl Object {
    fn apply_transform(&mut self, transform: &Transformation) {
        match self.shape {
            Shape::Group(ref mut g) => {
                g.children_mut().iter_mut().for_each(|child| {
                    child.apply_transform(transform);
                });
                self.transform = Transformation::new_transform();
            }
            _ => {
                self.transform.apply_transform(transform);
            }
        };
        self.bounds = self.bounds.transform(transform);
    }

    pub fn normal_at(&self, world_point: Point, hit: Intersection) -> Vector {
        let local_point = self.world_to_object(world_point);
        let local_normal = self.shape.normal_at(local_point, hit);
        self.normal_to_world(local_normal)
    }

    pub fn intersects(&self, r: &Ray) -> Intersections {
        let r = if !matches!(self.shape, Shape::Group(_)) {
            &r.transform(self.transform.inverse().unwrap())
        } else {
            r
        };
        self.shape.intersects(self, r)
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

    pub fn world_to_object(&self, point: Point) -> Point {
        self.transform.inverse().unwrap() * &point
    }

    pub fn normal_to_world(&self, normal: Vector) -> Vector {
        let normal = self.transform.inverse_transposed().unwrap() * &normal;
        normal.normalize()
    }

    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    pub fn bounds(&self) -> &Bounds {
        &self.bounds
    }

    pub fn normal(&self) -> Option<Vector> {
        self.shape.normal()
    }

    pub fn e1(&self) -> Option<Vector> {
        self.shape.e1()
    }

    pub fn e2(&self) -> Option<Vector> {
        self.shape.e2()
    }

    pub fn p1(&self) -> Option<Point> {
        self.shape.p1()
    }

    pub fn p2(&self) -> Option<Point> {
        self.shape.p2()
    }

    pub fn p3(&self) -> Option<Point> {
        self.shape.p3()
    }

    pub fn n1(&self) -> Option<Vector> {
        self.shape.n1()
    }

    pub fn n2(&self) -> Option<Vector> {
        self.shape.n2()
    }

    pub fn n3(&self) -> Option<Vector> {
        self.shape.n3()
    }

    pub fn group_mut(&mut self) -> Option<&mut Group> {
        self.shape.group_mut()
    }

    pub fn group(&self) -> Option<&Group> {
        self.shape.group()
    }

    pub fn remove_child(&mut self, child: &Object) {
        self.shape.remove_child(child);
    }

    pub fn divide(&mut self, threshold: usize) {
        self.shape.divide(threshold);
    }
}

#[cfg(test)]
mod tests {

    use std::f64::consts::PI;

    use crate::{matrix::Matrix, tuples::Tuple};

    use super::*;

    #[test]
    fn default_transformation() {
        let s = ObjectBuilder::new_test_shape().build();
        assert_eq!(s.transform.matrix, Matrix::identity(4));
    }

    #[test]
    fn changing_transformation() {
        let t = Transformation::new_transform().translation(2.0, 3.0, 4.0);
        let s = ObjectBuilder::new_test_shape()
            .with_transform(t.clone())
            .build();
        assert_eq!(s.transform, t);
    }

    #[test]
    fn default_material() {
        let s = ObjectBuilder::new_test_shape().build();
        let m = s.material.clone();
        assert_eq!(m, Material::new());
    }

    #[test]
    fn assigning_a_material() {
        let mut s = ObjectBuilder::new_test_shape().build();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.material = m.clone();
        assert_eq!(s.material, m);
    }

    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let s = ObjectBuilder::new_test_shape()
            .with_transform(Transformation::new_transform().translation(0.0, 1.0, 0.0))
            .build();
        let n = s.normal_at(
            Point::new(0.0, 1.70711, -0.70711),
            Intersection::new(1.0, &s),
        );
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
            .build();
        let n = s.normal_at(
            Point::new(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0),
            Intersection::new(1.0, &s),
        );
        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn creating_a_new_group() {
        let g = ObjectBuilder::new_group().build();
        assert!(g.group().unwrap().is_empty());
        assert_eq!(g.transform, Transformation::new_transform());
    }

    #[test]
    fn adding_a_child_to_a_group() {
        let s = ObjectBuilder::new_test_shape().build();
        let g = ObjectBuilder::new_group().add_child(s.clone()).build();
        assert!(!g.group().unwrap().is_empty());
        assert!(g.group().unwrap().contains(&s));
    }

    #[test]
    fn intersecting_a_ray_with_an_empty_group() {
        let g = ObjectBuilder::new_group().build();
        let r = Ray::new(Point::zero(), Vector::z_norm());
        let xs = g.intersects(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersecting_a_ray_with_an_nonempty_group() {
        let s1 = ObjectBuilder::new_sphere().build();
        let s2 = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().translation(0.0, 0.0, -3.0))
            .build();
        let s3 = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().translation(5.0, 0.0, 0.0))
            .build();
        let g = ObjectBuilder::new_group()
            .add_child(s1.clone())
            .add_child(s2.clone())
            .add_child(s3.clone())
            .build();

        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_norm());
        let xs = g.intersects(&r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].object, &s2);
        assert_eq!(xs[1].object, &s2);
        assert_eq!(xs[2].object, &s1);
        assert_eq!(xs[3].object, &s1);
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let s = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().translation(5.0, 0.0, 0.0))
            .build();
        let g = ObjectBuilder::new_group()
            .with_transform(Transformation::new_transform().scaling(2.0, 2.0, 2.0))
            .add_child(s)
            .build();
        let r = Ray::new(Point::new(10.0, 0.0, -10.0), Vector::z_norm());
        let xs = g.intersects(&r);
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn intersecting_a_nested_transformed_group() {
        let s = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().translation(5.0, 0.0, 0.0))
            .build();
        let g1 = ObjectBuilder::new_group().add_child(s).build();
        let g2 = ObjectBuilder::new_group()
            .with_transform(Transformation::new_transform().scaling(2.0, 2.0, 2.0))
            .add_child(g1)
            .build();
        let r = Ray::new(Point::new(10.0, 0.0, -10.0), Vector::z_norm());
        let xs = g2.intersects(&r);
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn intersecting_another_nested_transformed_group() {
        let s = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().translation(5.0, 0.0, 0.0))
            .build();
        let g1 = ObjectBuilder::new_group()
            .with_transform(Transformation::new_transform().scaling(2.0, 2.0, 2.0))
            .add_child(s)
            .build();
        let g2 = ObjectBuilder::new_group().add_child(g1).build();
        let r = Ray::new(Point::new(10.0, 0.0, -10.0), Vector::z_norm());
        let xs = g2.intersects(&r);
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn converting_a_point_from_world_to_object_space() {
        let s = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().translation(5.0, 0.0, 0.0))
            .build();
        let g2 = ObjectBuilder::new_group()
            .with_transform(Transformation::new_transform().scaling(2.0, 2.0, 2.0))
            .add_child(s.clone())
            .build();
        let g = ObjectBuilder::new_group()
            .with_transform(Transformation::new_transform().rotation_y(PI / 2.0))
            .add_child(g2.clone())
            .build();
        let s = &g.group().unwrap().children()[0].group().unwrap().children()[0];
        let p = s.world_to_object(Point::new(-2.0, 0.0, -10.0));
        assert_eq!(p, Point::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn converting_a_normal_from_object_to_world_space() {
        let s = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().translation(5.0, 0.0, 0.0))
            .build();
        let g2 = ObjectBuilder::new_group()
            .with_transform(Transformation::new_transform().scaling(1.0, 2.0, 3.0))
            .add_child(s.clone())
            .build();
        let g = ObjectBuilder::new_group()
            .with_transform(Transformation::new_transform().rotation_y(PI / 2.0))
            .add_child(g2.clone())
            .build();
        let s = &g.group().unwrap().children()[0].group().unwrap().children()[0];
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
            .build();
        let g2 = ObjectBuilder::new_group()
            .with_transform(Transformation::new_transform().scaling(1.0, 2.0, 3.0))
            .add_child(s.clone())
            .build();
        let g = ObjectBuilder::new_group()
            .with_transform(Transformation::new_transform().rotation_y(PI / 2.0))
            .add_child(g2.clone())
            .build();

        let s = &g.group().unwrap().children()[0].group().unwrap().children()[0];
        let n = s.normal_at(
            Point::new(1.7321, 1.1547, -5.5774),
            Intersection::new(1.0, &s),
        );
        assert_eq!(n, Vector::new(0.2857, 0.42854, -0.85716));
    }

    #[test]
    fn ray_intersects_bounds_in_a_non_transformed_shape() {
        let c = ObjectBuilder::new_cube().build();
        let g = ObjectBuilder::new_group().add_child(c).build();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_norm());
        let xs = g.intersects(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn ray_intersects_bounds_in_a_transformed_shape() {
        let c = ObjectBuilder::new_cube()
            .with_transform(Transformation::new_transform().translation(-5.0, 0.0, 0.0))
            .build();
        let g = ObjectBuilder::new_group().add_child(c).build();
        let r = Ray::new(Point::new(-5.0, 0.0, -5.0), Vector::z_norm());
        let xs = g.intersects(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn ray_intersects_bounds_in_multiple_transformed_shapes() {
        let c1 = ObjectBuilder::new_cube()
            .with_transform(Transformation::new_transform().translation(-5.0, 0.0, 0.0))
            .build();
        let c2 = ObjectBuilder::new_cube()
            .with_transform(Transformation::new_transform().translation(5.0, 0.0, 0.0))
            .build();
        let g = ObjectBuilder::new_group()
            .add_child(c1)
            .add_child(c2)
            .build();
        let r1 = Ray::new(Point::new(-5.0, 0.0, -5.0), Vector::z_norm());
        let r2 = Ray::new(Point::new(5.0, 0.0, -5.0), Vector::z_norm());
        let r0 = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_norm());
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

    #[test]
    fn querying_a_shape_s_bounding_box_in_its_parent_space() {
        let s = ObjectBuilder::new_sphere()
            .with_transform(
                Transformation::new_transform()
                    .scaling(0.5, 2.0, 4.0)
                    .translation(1.0, -3.0, 5.0),
            )
            .build();
        let b = s.bounds();
        assert_eq!(b.min(), &Point::new(0.5, -5.0, 1.0));
        assert_eq!(b.max(), &Point::new(1.5, -1.0, 9.0));
    }

    #[test]
    fn subdividing_a_primitive_does_nothing() {
        let mut s = ObjectBuilder::new_sphere().build();
        s.divide(1);
        assert!(matches!(s.shape(), Shape::Sphere(_)));
    }

    #[test]
    fn subdividing_a_group_partitions_its_children() {
        let s1 = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().translation(-2.0, -2.0, 0.0))
            .build();
        let s2 = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().translation(-2.0, 2.0, 0.0))
            .build();
        let s3 = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().scaling(4.0, 4.0, 4.0))
            .build();
        let mut g = ObjectBuilder::new_group()
            .add_child(s1.clone())
            .add_child(s2.clone())
            .add_child(s3.clone())
            .build();
        g.divide(1);
        assert_eq!(g.group().unwrap().children().len(), 2);
        assert_eq!(g.group().unwrap().children()[0], s3);
        let sub_g = g.group().unwrap().children()[1].clone();
        assert!(matches!(sub_g.shape, Shape::Group(_)));
        assert_eq!(sub_g.group().unwrap().len(), 2);
        assert_eq!(
            sub_g.group().unwrap().children()[0]
                .group()
                .unwrap()
                .children()[0],
            s1
        );
        assert_eq!(
            sub_g.group().unwrap().children()[1]
                .group()
                .unwrap()
                .children()[0],
            s2
        );
    }

    #[test]
    fn subdividing_a_group_with_too_few_children() {
        let s1 = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().translation(-2.0, 0.0, 0.0))
            .build();
        let s2 = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().translation(2.0, 1.0, 0.0))
            .build();
        let s3 = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().translation(2.0, -1.0, 0.0))
            .build();
        let sub_g = ObjectBuilder::new_group()
            .add_child(s1.clone())
            .add_child(s2.clone())
            .add_child(s3.clone())
            .build();
        let s4 = ObjectBuilder::new_sphere().build();
        let mut g = ObjectBuilder::new_group()
            .add_child(sub_g.clone())
            .add_child(s4.clone())
            .build();
        g.divide(3);
        assert_eq!(g.group().unwrap().len(), 2);
        let sub_g = g.group().unwrap().children()[0].clone();
        assert!(matches!(sub_g.shape, Shape::Group(_)));
        assert_eq!(sub_g.group().unwrap().len(), 2);
        let (sub_g_0, sub_g_1) = (
            sub_g.group().unwrap().children()[0].clone(),
            sub_g.group().unwrap().children()[1].clone(),
        );
        assert!(matches!(sub_g_0.shape, Shape::Group(_)));
        assert!(matches!(sub_g_1.shape, Shape::Group(_)));
        assert_eq!(sub_g_0.group().unwrap().len(), 1);
        assert_eq!(sub_g_1.group().unwrap().len(), 2);
        assert_eq!(sub_g_0.group().unwrap().children()[0], s1);
        assert_eq!(sub_g_1.group().unwrap().children()[0], s2);
        assert_eq!(sub_g_1.group().unwrap().children()[1], s3);
    }
}
