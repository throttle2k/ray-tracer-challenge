use crate::{bounds::Bounds, intersections::Intersections, rays::Ray};

use super::Object;

#[derive(Debug, Clone, PartialEq)]
pub enum CSGKind {
    Union,
    Intersection,
    Difference,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CSG {
    kind: CSGKind,
    left: Box<Object>,
    right: Box<Object>,
}

impl CSG {
    pub fn new(kind: CSGKind, left: Object, right: Object) -> Self {
        Self {
            kind,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn left(&self) -> &Object {
        self.left.as_ref()
    }

    pub fn right(&self) -> &Object {
        self.right.as_ref()
    }

    pub fn bounds(&self) -> crate::bounds::Bounds {
        vec![self.left(), self.right()]
            .iter()
            .fold(Bounds::default(), |bound, o| bound + o.bounds().clone())
    }

    pub fn normal_at(
        &self,
        local_point: crate::tuples::points::Point,
    ) -> crate::tuples::vectors::Vector {
        todo!()
    }

    pub fn intersection_allowed(
        &self,
        left_hit: bool,
        inside_left: bool,
        inside_right: bool,
    ) -> bool {
        match self.kind {
            CSGKind::Union => (left_hit && !inside_right) || (!left_hit && !inside_left),
            CSGKind::Intersection => (left_hit && inside_right) || (!left_hit && inside_left),
            CSGKind::Difference => (left_hit && !inside_right) || (!left_hit && inside_left),
        }
    }

    pub fn intersects(&self, _object: &Object, ray: &Ray) -> Intersections {
        let left_xs = self.left().intersects(ray);
        let right_xs = self.right().intersects(ray);
        let mut xs: Intersections = left_xs
            .clone()
            .into_iter()
            .chain(right_xs.clone().into_iter())
            .collect();
        xs.sort_unstable_by(|i1, i2| i1.t.total_cmp(&i2.t));
        self.filter_intersections(&xs)
    }

    pub fn filter_intersections<'a>(&'a self, xs: &Intersections<'a>) -> Intersections {
        let (_, _, result) = xs.iter().fold(
            (false, false, Intersections::new()),
            |(mut inside_left, mut inside_right, mut xs), i| {
                let left_hit = self.left.includes(i.object);
                if self.intersection_allowed(left_hit, inside_left, inside_right) {
                    xs.push(*i);
                }
                if left_hit {
                    inside_left = !inside_left;
                } else {
                    inside_right = !inside_right;
                }

                (inside_left, inside_right, xs)
            },
        );
        result
    }
}

#[cfg(test)]
mod tests {
    use yare::parameterized;

    use crate::{
        intersections::{Intersection, Intersections},
        shapes::ObjectBuilder,
        tuples::vectors::Vector,
    };

    use super::*;

    #[test]
    fn csg_is_created_with_an_operation_and_two_shapes() {
        let s1 = ObjectBuilder::new_sphere().build();
        let s2 = ObjectBuilder::new_cube().build();
        let csg = CSG::new(CSGKind::Union, s1.clone(), s2.clone());
        assert_eq!(csg.left.as_ref(), &s1);
        assert_eq!(csg.right.as_ref(), &s2);
    }

    #[parameterized(
        union_1 = {CSGKind::Union, true, true, true, false},
        union_2 = {CSGKind::Union, true, true, false, true},
        union_3 = {CSGKind::Union, true, false, true, false},
        union_4 = {CSGKind::Union, true, false, false, true},
        union_5 = {CSGKind::Union, false, true, true, false},
        union_6 = {CSGKind::Union, false, true, false, false},
        union_7 = {CSGKind::Union, false, false, true, true},
        union_8 = {CSGKind::Union, false, false, false, true},
        intersection_1 = {CSGKind::Intersection, true, true, true, true},
        intersection_2 = {CSGKind::Intersection, true, true, false, false},
        intersection_3 = {CSGKind::Intersection, true, false, true, true},
        intersection_4 = {CSGKind::Intersection, true, false, false, false},
        intersection_5 = {CSGKind::Intersection, false, true, true, true},
        intersection_6 = {CSGKind::Intersection, false, true, false, true},
        intersection_7 = {CSGKind::Intersection, false, false, true, false},
        intersection_8 = {CSGKind::Intersection, false, false, false, false},
        difference_1 = {CSGKind::Difference, true, true, true, false},
        difference_2 = {CSGKind::Difference, true, true, false, true},
        difference_3 = {CSGKind::Difference, true, false, true, false},
        difference_4 = {CSGKind::Difference, true, false, false, true},
        difference_5 = {CSGKind::Difference, false, true, true, true},
        difference_6 = {CSGKind::Difference, false, true, false, true},
        difference_7 = {CSGKind::Difference, false, false, true, false},
        difference_8 = {CSGKind::Difference, false, false, false, false},
    )]
    fn evaluating_rule_for_csg_operation(
        operation: CSGKind,
        lhit: bool,
        inl: bool,
        inr: bool,
        result: bool,
    ) {
        let s1 = ObjectBuilder::new_sphere().build();
        let s2 = ObjectBuilder::new_cube().build();
        let c = CSG::new(operation, s1.clone(), s2.clone());
        assert_eq!(c.intersection_allowed(lhit, inl, inr), result);
    }

    #[parameterized(
        union = {CSGKind::Union, 0.0, 3.0},
        intersection = {CSGKind::Intersection, 1.0, 2.0},
        difference = {CSGKind::Difference, 0.0, 1.0}
    )]
    fn filtering_a_list_of_intersections(operation: CSGKind, x0: f64, x1: f64) {
        let s1 = ObjectBuilder::new_sphere().build();
        let s2 = ObjectBuilder::new_cube().build();
        let c = CSG::new(operation, s1.clone(), s2.clone());
        let mut xs = Intersections::new();
        xs.push(Intersection::new(0.0, &s1));
        xs.push(Intersection::new(1.0, &s2));
        xs.push(Intersection::new(2.0, &s1));
        xs.push(Intersection::new(3.0, &s2));
        let result = c.filter_intersections(&xs);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].t, x0);
        assert_eq!(result[1].t, x1);
    }
}
