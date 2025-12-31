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
                BvhNode::Leaf { object: obj, bbox }
            }
            2 => {
                let (left_objs, right_objs) = objects.split_at_mut(1);
                let left_node = Box::new(Self::build_from_slice(left_objs));
                let right_node = Box::new(Self::build_from_slice(right_objs));
                let bbox = Aabb::surrounding_box(&left_node.bbox(), &right_node.bbox());
                BvhNode::Node {
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
                BvhNode::Node {
                    left: left_node,
                    right: right_node,
                    bbox,
                }
            }
        }
    }
}

impl Hittable for BvhNode {
    fn intersect(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        match self {
            BvhNode::Leaf { object, bbox } => {
                if bbox.intersect(r, ray_t) && object.intersect(r, ray_t, rec) {
                    return true;
                }
                false
            }
            BvhNode::Node { left, right, bbox } => {
                if !bbox.intersect(r, ray_t) {
                    return false;
                }
                let mut hit_any = false;
                let mut temp_rec = HitRecord::new();
                let mut search_interval = ray_t;

                if left.intersect(r, search_interval, &mut temp_rec) {
                    hit_any = true;
                    *rec = temp_rec.clone();
                    search_interval.max = temp_rec.t;
                }
                if right.intersect(r, search_interval, &mut temp_rec) {
                    hit_any = true;
                    *rec = temp_rec;
                }
                hit_any
            }
        }
    }
}

impl Bounded for BvhNode {
    /// Get bounding box of this node.
    fn bbox(&self) -> Aabb {
        match self {
            BvhNode::Leaf { bbox, .. } => *bbox,
            BvhNode::Node { bbox, .. } => *bbox,
        }
    }
}
