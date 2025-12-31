use std::mem::swap;

use crate::{
    interval::Interval,
    math::{Point3, Ray},
};

#[derive(Clone, Copy)]
/// Axis-Aligned Bounding Box.
pub struct Aabb {
    // pub min: Point3,
    // pub max: Point3,
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    /// Create AABB based on the xyz of the `Interval` structure.
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    /// Create AABB from min and max points.
    pub fn from_points(p0: Point3, p1: Point3) -> Self {
        // Ensure each axis interval is ordered so callers don't need to pre-sort inputs.
        let x = Interval::new(p0.x.min(p1.x), p0.x.max(p1.x));
        let y = Interval::new(p0.y.min(p1.y), p0.y.max(p1.y));
        let z = Interval::new(p0.z.min(p1.z), p0.z.max(p1.z));
        Self { x, y, z }
    }

    /// Create surrounding box that contains two AABBs.
    pub fn surrounding_box(a: &Aabb, b: &Aabb) -> Aabb {
        let x_min = a.x.min.min(b.x.min);
        let y_min = a.y.min.min(b.y.min);
        let z_min = a.z.min.min(b.z.min);
        let x_max = a.x.max.max(b.x.max);
        let y_max = a.y.max.max(b.y.max);
        let z_max = a.z.max.max(b.z.max);
        let x = Interval::new(x_min, x_max);
        let y = Interval::new(y_min, y_max);
        let z = Interval::new(z_min, z_max);
        Aabb::new(x, y, z)
    }

    pub fn axis_interval(&self, axis_index: usize) -> Interval {
        match axis_index {
            0 => self.x,
            1 => self.y,
            _ => self.z,
        }
    }

    /// Check if ray intersects with AABB.
    pub fn intersect(&self, r: &Ray, ray_t: Interval) -> bool {
        let mut bounds = ray_t;
        for axis in 0..3 {
            let interval = self.axis_interval(axis);
            let inv_d = 1.0 / r.direction[axis];
            let mut t0 = (interval.min - r.origin[axis]) * inv_d;
            let mut t1 = (interval.max - r.origin[axis]) * inv_d;
            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }
            bounds.min = bounds.min.max(t0);
            bounds.max = bounds.max.min(t1);
            if bounds.max <= bounds.min {
                return false;
            }
        }
        true
    }

    /// Ensure no side is narrower than delta, padding if necessary
    pub fn padding_to_minimal(mut self) -> Self {
        let delta = 0.001;
        if self.x.size() < delta {
            self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z.expand(delta);
        }
        self
    }

    /// Get the longest axis of the AABB: 0 for x, 1 for y, 2 for z.
    pub fn longest_axis(&self) -> usize {
        if self.x.size() >= self.y.size() && self.x.size() >= self.z.size() {
            0
        } else if self.y.size() >= self.z.size() {
            1
        } else {
            2
        }
    }
}
