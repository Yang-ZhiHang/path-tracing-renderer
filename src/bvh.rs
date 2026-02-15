use std::cmp::Ordering;

use crate::aabb::Aabb;
use crate::interval::Interval;
use crate::math::{Axis, Ray};
use crate::object::Object;
use crate::shape::{Bounded, HitRecord, Hittable};

/// A node in the Bounding Volume Hierarchy. Used to accelerate ray intersection: O(n) -> O(log_n)
pub enum BvhNode {
    Leaf {
        object: Object,
        bbox: Aabb,
    },
    Node {
        left: Box<BvhNode>,
        right: Box<BvhNode>,
        bbox: Aabb,
    },
}

impl BvhNode {
    /// Build BVH from list of objects.
    pub fn build(objects: Vec<Object>) -> Self {
        let mut objs = objects;
        Self::build_from_slice(&mut objs)
    }

    /// Compare the min value of AABB in given axis index.
    pub fn box_compare(a: Aabb, b: Aabb, axis: Axis) -> Ordering {
        let a_axis_interval = a.axis_interval(axis);
        let b_axis_interval = b.axis_interval(axis);
        a_axis_interval
            .min
            .partial_cmp(&b_axis_interval.min)
            .unwrap_or(Ordering::Equal)
    }

    /// Build BVH from slice of objects.
    fn build_from_slice(objects: &mut [Object]) -> Self {
        // Compute the aabb of all objects (the biggest aabb).
        // Then, sort objects and split into two halves (according to longest axis).
        let (first, rest) = objects.split_first().unwrap();
        let mut bbox = first.bbox();
        for obj in rest {
            bbox = Aabb::surrounding_box(&bbox, &obj.bbox());
        }
        let axis = bbox.longest_axis();
        objects.sort_by(|a, b| {
            let box_a = a.bbox();
            let box_b = b.bbox();
            Self::box_compare(box_a, box_b, axis)
        });

        match objects.len() {
            0 => panic!("BVH build called with empty object list"),
            1 => {
                let obj = objects[0].clone();
                let bbox = obj.bbox();
                Self::Leaf { object: obj, bbox }
            }
            2 => {
                let (left_objs, right_objs) = objects.split_at_mut(1);
                let left_node = Box::new(Self::build_from_slice(left_objs));
                let right_node = Box::new(Self::build_from_slice(right_objs));
                Self::Node {
                    left: left_node,
                    right: right_node,
                    bbox,
                }
            }
            _ => {
                let mid = objects.len() / 2;
                let (left_objs, right_objs) = objects.split_at_mut(mid);
                let left_node = Box::new(Self::build_from_slice(left_objs));
                let right_node = Box::new(Self::build_from_slice(right_objs));
                Self::Node {
                    left: left_node,
                    right: right_node,
                    bbox,
                }
            }
        }
    }
}

impl Hittable for BvhNode {
    fn intersect(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        match self {
            Self::Leaf { object, bbox } => {
                if !bbox.intersect(r, ray_t) {
                    return None;
                }
                object.intersect(r, ray_t)
            }
            Self::Node { left, right, bbox } => {
                if !bbox.intersect(r, ray_t) {
                    return None;
                }

                let hit_left = left.intersect(r, ray_t);
                let t_max = hit_left.as_ref().map_or(ray_t.max, |rec| rec.t);
                let hit_right = right.intersect(r, Interval::new(ray_t.min, t_max));

                hit_right.or(hit_left)
            }
        }
    }
}

impl Bounded for BvhNode {
    /// Get bounding box of this node.
    fn bbox(&self) -> Aabb {
        match self {
            Self::Leaf { bbox, .. } => *bbox,
            Self::Node { bbox, .. } => *bbox,
        }
    }
}
