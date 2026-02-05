use std::cmp::Ordering;

use crate::aabb::Aabb;
use crate::interval::Interval;
use crate::math::Ray;
use crate::object::Object;
use crate::shape::{Bounded, HitRecord, Hittable};

/// A node in the Bounding Volume Hierarchy. Used to accelerate ray intersection: O(n) -> O(log n)
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
    pub fn box_compare(a: Aabb, b: Aabb, axis_index: usize) -> Ordering {
        let a_axis_interval = a.axis_interval(axis_index);
        let b_axis_interval = b.axis_interval(axis_index);
        a_axis_interval
            .min
            .partial_cmp(&b_axis_interval.min)
            .unwrap_or(Ordering::Equal)
    }

    /// Compare the min value of AABB in x axis.
    pub fn box_x_compare(a: Aabb, b: Aabb) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    /// Compare the min value of AABB in y axis.
    pub fn box_y_compare(a: Aabb, b: Aabb) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    /// Compare the min value of AABB in z axis.
    pub fn box_z_compare(a: Aabb, b: Aabb) -> Ordering {
        Self::box_compare(a, b, 2)
    }

    /// Build BVH from slice of objects.
    fn build_from_slice(objects: &mut [Object]) -> Self {
        let (first, rest) = objects.split_first().unwrap();
        let mut bbox = first.bbox();
        for obj in rest {
            bbox = Aabb::surrounding_box(&bbox, &obj.bbox());
        }
        let axis_index = bbox.longest_axis();
        let comparator = match axis_index {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };
        objects.sort_by(|a, b| {
            let box_a = a.bbox();
            let box_b = b.bbox();
            comparator(box_a, box_b)
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
                let bbox = Aabb::surrounding_box(&left_node.bbox(), &right_node.bbox());
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
                let bbox = Aabb::surrounding_box(&left_node.bbox(), &right_node.bbox());
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
