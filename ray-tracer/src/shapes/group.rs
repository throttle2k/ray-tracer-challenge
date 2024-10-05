use std::ops::Deref;

use crate::{
    intersections::Intersections,
    rays::Ray,
    tuples::{points::Point, vectors::Vector},
    REGISTRY,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    children: Vec<usize>,
}

impl Group {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn normal_at(&self, _object_point: Point) -> Vector {
        unreachable!()
    }

    pub fn intersects(&self, ray: Ray) -> Intersections {
        let registry = REGISTRY.read().unwrap();
        let mut xs = Intersections::new();
        for child_id in self.children.iter() {
            let child = registry.get_object(*child_id).unwrap();
            let child_xs = child.intersects(&ray);
            child_xs.iter().for_each(|i| xs.push(*i));
        }
        xs.sort_unstable_by(|a, b| a.t.total_cmp(&b.t));
        xs
    }

    pub fn add_child(&mut self, object_id: usize) {
        self.children.push(object_id);
    }
}

impl Deref for Group {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}
