use std::f64::{INFINITY, NEG_INFINITY};

use crate::{
    bounds::Bounds,
    intersections::Intersections,
    rays::Ray,
    shapes::Object,
    tuples::{points::Point, Tuple},
};

const MAX_SIZE: f64 = 1024.0;
const MIN_SIZE: f64 = 0.001;

#[derive(Debug)]
pub struct Octree<'a> {
    region: Bounds,
    objects: Vec<&'a Object>,
    children: [Option<Box<Octree<'a>>>; 8],
}

impl<'a> Octree<'a> {
    pub fn with_region(mut self, region: Bounds) -> Self {
        self.region = region;
        self
    }

    pub fn with_objects(mut self, objects: Vec<&'a Object>) -> Self {
        self.objects = objects;
        self
    }

    pub fn has_children(&self) -> bool {
        self.children.iter().any(|child| child.is_some())
    }

    pub fn intersects(&self, r: &Ray) -> Intersections {
        if self.objects.len() == 0 && !self.has_children() {
            Intersections::new()
        } else {
            let mut intersections = Intersections::new();
            for obj in self.objects.clone() {
                match obj.shape() {
                    crate::shapes::Shape::Group(_) => {}
                    _ => {
                        let mut xs = obj.intersects(r);
                        for i in xs.iter_mut() {
                            intersections.push(*i);
                        }
                    }
                }
            }
            for idx in 0..=7 {
                if let Some(child) = &self.children[idx] {
                    if child.region.intersects(&r) {
                        let mut xs = child.intersects(r);
                        for i in xs.iter_mut() {
                            intersections.push(*i);
                        }
                    }
                }
            }
            intersections
        }
    }

    pub fn build(mut self) -> Option<Octree<'a>> {
        if self.objects.len() == 1 {
            return None;
        }

        let dimensions = *self.region.max() - *self.region.min();
        if dimensions.x() <= MIN_SIZE && dimensions.y() <= MIN_SIZE && dimensions.z() <= MIN_SIZE {
            return None;
        }

        let dimensions = *self.region.max() - *self.region.min();
        let half = dimensions / 2.0;
        let center = *self.region.min() + half;
        let octants = [
            Bounds::new(*self.region.min(), center),
            Bounds::new(
                Point::new(center.x(), self.region.min().y(), self.region.min().z()),
                Point::new(self.region.max().x(), center.y(), center.z()),
            ),
            Bounds::new(
                Point::new(center.x(), self.region.min().y(), center.z()),
                Point::new(self.region.max().x(), center.y(), self.region.max().z()),
            ),
            Bounds::new(
                Point::new(self.region.min().x(), self.region.min().y(), center.z()),
                Point::new(center.x(), center.y(), self.region.max().z()),
            ),
            Bounds::new(
                Point::new(self.region.min().x(), center.y(), self.region.min().z()),
                Point::new(center.x(), self.region.max().y(), center.z()),
            ),
            Bounds::new(
                Point::new(center.x(), center.y(), self.region.min().z()),
                Point::new(self.region.max().x(), self.region.max().y(), center.z()),
            ),
            Bounds::new(center, *self.region.max()),
            Bounds::new(
                Point::new(self.region.min().x(), center.y(), center.z()),
                Point::new(center.x(), self.region.max().y(), self.region.max().z()),
            ),
        ];
        let mut oct_vecs: [Vec<&Object>; 8] = Default::default();
        let mut to_remove: Vec<&Object> = Vec::new();

        self.objects.iter().for_each(|obj| match obj.shape() {
            crate::shapes::Shape::Group(_) => {}
            _ => {
                if obj.bounds().min().x() > NEG_INFINITY
                    && obj.bounds().min().y() > NEG_INFINITY
                    && obj.bounds().min().z() > NEG_INFINITY
                    && obj.bounds().max().x() < INFINITY
                    && obj.bounds().max().y() < INFINITY
                    && obj.bounds().max().z() < INFINITY
                {
                    for idx in 0..=7 {
                        if octants[idx].contains(&obj.bounds()) {
                            oct_vecs[idx].push(obj);
                            to_remove.push(obj);
                        }
                    }
                }
            }
        });
        self.objects.retain(|i| !to_remove.contains(i));
        for idx in 0..=7 {
            match oct_vecs[idx].len() {
                0 => self.children[idx] = None,
                _ => {
                    self.children[idx] = Self::default()
                        .with_region(octants[idx])
                        .with_objects(oct_vecs[idx].clone())
                        .build()
                        .map(Box::new);
                }
            }
        }
        Some(self)
    }
}

impl<'a> Default for Octree<'a> {
    fn default() -> Self {
        Self {
            region: Bounds::new(
                Point::new(-MAX_SIZE, -MAX_SIZE, -MAX_SIZE),
                Point::new(MAX_SIZE, MAX_SIZE, MAX_SIZE),
            ),
            objects: Vec::new(),
            children: [None, None, None, None, None, None, None, None],
        }
    }
}
