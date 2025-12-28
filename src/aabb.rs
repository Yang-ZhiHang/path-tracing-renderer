use crate::{
    math::{Point3, Ray},
    shape::{HitRecord, Hittable},
};

#[derive(Clone, Copy, Debug)]
/// Axis-Aligned Bounding Box.
pub struct Aabb {
    pub min: Point3,
    pub max: Point3,
}

impl Aabb {
    /// Create AABB from min and max points.
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    /// Create surrounding box that contains two AABBs.
    pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
        let min: Point3 = box0.min.min(box1.min);
        let max: Point3 = box0.max.max(box1.max);
        Aabb::new(min, max)
    }
}

impl Hittable for Aabb {
    fn intersect(&self, r: &Ray, mut t_min: f32, mut t_max: f32, _rec: &mut HitRecord) -> bool {
        for axis in 0..3 {
            let inv_d = 1.0 / r.direction[axis];
            let t0 = (self.min[axis] - r.origin[axis]) * inv_d;
            let t1 = (self.max[axis] - r.origin[axis]) * inv_d;
            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}
