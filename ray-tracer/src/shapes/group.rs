use std::ops::Deref;

use crate::{
    bounds::Bounds,
    intersections::Intersections,
    rays::Ray,
    tuples::{points::Point, vectors::Vector},
    REGISTRY,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    children: Vec<usize>,
    bounds: Bounds,
}

impl Group {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            bounds: Bounds::default(),
        }
    }

    pub fn normal_at(&self, _object_point: Point) -> Vector {
        unreachable!()
    }

    pub fn intersects(&self, ray: Ray) -> Intersections {
        let mut xs = Intersections::new();
        if self.bounds().intersects(&ray) {
            let registry = REGISTRY.read().unwrap();
            for child_id in self.children.iter() {
                let child = registry.get_object(*child_id).unwrap();
                let child_xs = child.intersects(&ray);
                child_xs.iter().for_each(|i| xs.push(*i));
            }
            xs.sort_unstable_by(|a, b| a.t.total_cmp(&b.t));
        }
        xs
    }

    pub fn add_child(&mut self, object_id: usize) {
        self.children.push(object_id);
    }

    pub fn bounds(&self) -> Bounds {
        let registry = REGISTRY.read().unwrap();
        self.children
            .iter()
            .fold(Bounds::default(), |bounds, object_id| {
                let object = registry.get_object(*object_id).unwrap();
                bounds + object.bounds()
            })
    }

    pub fn children(&self) -> &[usize] {
        &self.children
    }
}

impl Deref for Group {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}
