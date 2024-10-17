use std::ops::Deref;

use crate::{
    bounds::Bounds,
    intersections::Intersections,
    rays::Ray,
    tuples::{points::Point, vectors::Vector},
};

use super::{Object, ObjectBuilder};

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    children: Vec<Object>,
    bounds: Bounds,
}

impl Default for Group {
    fn default() -> Self {
        Self {
            children: Vec::new(),
            bounds: Bounds::default(),
        }
    }
}

impl Group {
    pub fn add_child(&mut self, object: Object) {
        self.children.push(object);
    }

    pub fn children(&self) -> &[Object] {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut Vec<Object> {
        self.children.as_mut()
    }

    pub fn normal_at(&self, _object_point: Point) -> Vector {
        unreachable!()
    }

    pub fn intersects<'a>(&'a self, _object: &'a Object, ray: &Ray) -> Intersections<'a> {
        let mut xs = Intersections::new();
        if self.bounds().intersects(&ray) {
            for child in self.children.iter() {
                let child_xs = child.intersects(&ray);
                child_xs.iter().for_each(|i| xs.push(*i));
            }
            xs.sort_unstable_by(|a, b| a.t.total_cmp(&b.t));
        }
        xs
    }

    pub fn bounds(&self) -> Bounds {
        self.children
            .iter()
            .fold(Bounds::default(), |bounds, child| {
                bounds + child.bounds().clone()
            })
    }

    pub fn remove_child(&mut self, child: &Object) {
        let idx = self
            .children
            .iter()
            .position(|current_child| current_child == child)
            .unwrap();
        self.children.remove(idx);
    }

    pub fn partition_children(&mut self) -> (Vec<Object>, Vec<Object>) {
        let mut left = Vec::new();
        let mut right = Vec::new();
        let (left_box, right_box) = self.bounds().split();
        let mut to_keep = Vec::new();
        for child in self.children.iter() {
            if left_box.contains(child.bounds()) {
                left.push(child.clone());
            } else if right_box.contains(child.bounds()) {
                right.push(child.clone());
            } else {
                to_keep.push(child.clone());
            }
        }
        self.children = to_keep;

        (left, right)
    }

    pub fn make_subgroup(&mut self, children_list: Vec<Object>) {
        let mut sub_g = ObjectBuilder::new_group();
        for child in children_list {
            sub_g = sub_g.add_child(child);
        }
        let sub_g = sub_g.build();
        self.add_child(sub_g);
    }

    pub fn divide(&mut self, threshold: usize) {
        if threshold <= self.children.len() {
            let (left, right) = self.partition_children();
            if !left.is_empty() {
                self.make_subgroup(left);
            }
            if !right.is_empty() {
                self.make_subgroup(right);
            }
        }
        for child in &mut self.children {
            child.divide(threshold);
        }
    }
}

impl Deref for Group {
    type Target = Vec<Object>;

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

#[cfg(test)]
mod tests {
    use crate::{shapes::ObjectBuilder, transformations::Transformation, tuples::Tuple};

    use super::*;

    #[test]
    fn a_group_has_a_bounding_box_that_contains_its_children() {
        let s = ObjectBuilder::new_sphere()
            .with_transform(
                Transformation::new_transform()
                    .scaling(2.0, 2.0, 2.0)
                    .translation(2.0, 5.0, -3.0),
            )
            .build();
        let c = ObjectBuilder::new_cylinder()
            .with_min(-2.0)
            .with_max(2.0)
            .with_transform(
                Transformation::new_transform()
                    .scaling(0.5, 1.0, 0.5)
                    .translation(-4.0, -1.0, 4.0),
            )
            .build();
        let g = ObjectBuilder::new_group().add_child(s).add_child(c).build();
        let b = g.bounds();
        assert_eq!(b.min(), &Point::new(-4.5, -3.0, -5.0));
        assert_eq!(b.max(), &Point::new(4.0, 7.0, 4.5));
    }

    #[test]
    fn intersecting_ray_group_does_not_test_children_if_box_is_missed() {
        let c = ObjectBuilder::new_test_shape().build();
        let g = ObjectBuilder::new_group().add_child(c).build();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::y_norm());
        let xs = g.intersects(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersecting_ray_group_tests_children_if_box_is_hit() {
        let c = ObjectBuilder::new_test_shape().build();
        let g = ObjectBuilder::new_group().add_child(c).build();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_norm());
        let xs = g.intersects(&r);
        assert!(!xs.is_empty());
    }

    #[test]
    fn partitioning_a_group_s_children() {
        let s1 = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().translation(-2.0, 0.0, 0.0))
            .build();
        let s2 = ObjectBuilder::new_sphere()
            .with_transform(Transformation::new_transform().translation(2.0, 0.0, 0.0))
            .build();
        let s3 = ObjectBuilder::new_sphere().build();
        let mut g = Group::default();
        g.add_child(s1.clone());
        g.add_child(s2.clone());
        g.add_child(s3.clone());
        let (left, right) = g.partition_children();
        assert_eq!(vec![s3], g.children);
        assert_eq!(vec![s1], left);
        assert_eq!(vec![s2], right);
    }

    #[test]
    fn make_a_subgroup_from_list_of_children() {
        let s1 = ObjectBuilder::new_sphere().build();
        let s2 = ObjectBuilder::new_sphere().build();
        let mut g = Group::default();
        g.make_subgroup(vec![s1.clone(), s2.clone()]);
        assert_eq!(g.children.len(), 1);
        let sub_g = g.children.get(0).unwrap();
        assert!(sub_g.group().is_some());
        assert_eq!(sub_g.group().unwrap().children(), vec![s1, s2]);
    }
}
