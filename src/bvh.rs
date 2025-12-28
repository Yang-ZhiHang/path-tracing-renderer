use crate::aabb::Aabb;
use crate::math::Ray;
use crate::object::Object;
use crate::shape::{HitRecord, Hittable};
use rand::random_range;
use std::cmp::Ordering;

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

    /// Build BVH from slice of objects.
    fn build_from_slice(objects: &mut [Object]) -> Self {
        let axis = random_range(0..3);
        objects.sort_by(|a, b| {
            let box_a = a.bounding_box();
            let box_b = b.bounding_box();
            box_a.min[axis]
                .partial_cmp(&box_b.min[axis])
                .unwrap_or(Ordering::Equal)
        });

        match objects.len() {
            0 => panic!("BVH build called with empty object list"),
            1 => {
                let obj = objects[0].clone();
                let bbox = obj.bounding_box();
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

    /// Get bounding box of this node.
    fn bbox(&self) -> Aabb {
        match self {
            BvhNode::Leaf { bbox, .. } => *bbox,
            BvhNode::Node { bbox, .. } => *bbox,
        }
    }
}

impl Hittable for BvhNode {
    fn intersect(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        match self {
            BvhNode::Leaf { object, bbox } => {
                if !bbox.intersect(r, t_min, t_max, rec) {
                    return false;
                }
                let mut local_rec = HitRecord::new();
                local_rec.material = Some(object.material.clone());
                if object.intersect(r, t_min, t_max, &mut local_rec) {
                    *rec = local_rec;
                    return true;
                }
                false
            }
            BvhNode::Node { left, right, bbox } => {
                if !bbox.intersect(r, t_min, t_max, rec) {
                    return false;
                }
                let mut hit_any = false;
                let mut closest_so_far = t_max;
                let mut temp_rec = HitRecord::new();

                if left.intersect(r, t_min, closest_so_far, &mut temp_rec) {
                    hit_any = true;
                    closest_so_far = temp_rec.t;
                    *rec = temp_rec.clone();
                }
                if right.intersect(r, t_min, closest_so_far, &mut temp_rec) {
                    hit_any = true;
                    *rec = temp_rec;
                }
                hit_any
            }
        }
    }
}
