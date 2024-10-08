use std::f64::{INFINITY, NEG_INFINITY};

use crate::{
    bounds::Bounds,
    intersections::Intersections,
    rays::Ray,
    tuples::{points::Point, Tuple},
    REGISTRY,
};

const MAX_SIZE: f64 = 1024.0;
const MIN_SIZE: f64 = 0.001;

#[derive(Debug)]
pub struct Octree {
    region: Bounds,
    objects: Vec<usize>,
    children: [Option<Box<Octree>>; 8],
}

impl Octree {
    pub fn with_region(mut self, region: Bounds) -> Self {
        self.region = region;
        self
    }

    pub fn with_objects(mut self, objects: Vec<usize>) -> Self {
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
            let registry = REGISTRY.read().unwrap();
            for obj in self.objects.as_slice() {
                let obj = registry.get_object(*obj).unwrap();
                println!("{:?}", &obj);
                if obj.bounds().min().x() == 0.0
                    || obj.bounds().min().y() == 0.0
                    || obj.bounds().min().z() == 0.0
                    || obj.bounds().max().x() == 0.0
                    || obj.bounds().max().z() == 0.0
                    || obj.bounds().max().y() == 0.0
                {
                    println!("ZERO")
                }
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
                    let mut xs = child.intersects(r);
                    for i in xs.iter_mut() {
                        intersections.push(*i);
                    }
                }
            }
            intersections
        }
    }

    pub fn build(mut self) -> Option<Octree> {
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
        let mut oct_vecs: [Vec<usize>; 8] = Default::default();
        let mut to_remove: Vec<usize> = Vec::new();

        let registry = REGISTRY.read().unwrap();
        for obj in self.objects.as_slice() {
            let obj = registry.get_object(*obj).unwrap();
            match obj.shape() {
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
                                println!(
                                    "Object {} contained in {:?}, {:?}",
                                    &obj.id,
                                    octants[idx].min(),
                                    octants[idx].max()
                                );
                                oct_vecs[idx].push(obj.id);
                                to_remove.push(obj.id);
                            } else {
                                println!(
                                    "Object {} not contained in {:?}, {:?}",
                                    &obj.id,
                                    octants[idx].min(),
                                    octants[idx].max()
                                );
                            }
                        }
                    }
                }
            }
        }
        self.objects.retain(|i| !to_remove.contains(i));
        for idx in 0..=7 {
            match oct_vecs[idx].len() {
                0 => self.children[idx] = None,
                _ => {
                    self.children[idx] = Self::default()
                        .with_region(octants[idx].clone())
                        .with_objects(oct_vecs[idx].clone())
                        .build()
                        .map(Box::new);
                }
            }
        }
        Some(self)
    }
}

impl Default for Octree {
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
