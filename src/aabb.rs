use std::mem::swap;

use crate::{
    interval::Interval,
    math::{Axis, Point3, Ray},
};

#[derive(Clone, Copy)]
/// Axis-Aligned Bounding Box.
pub struct Aabb {
    /// The interval in x axis.
    pub x: Interval,

    /// The interval in y axis.
    pub y: Interval,

    /// The interval in z axis.
    pub z: Interval,
}

impl Aabb {
    /// Create AABB based on the xyz of the `Interval` structure.
    pub const fn new(x: Interval, y: Interval, z: Interval) -> Self {
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
    pub fn surrounding_box(a: &Self, b: &Self) -> Self {
        Self::new(a.x.union(&b.x), a.y.union(&b.y), a.z.union(&b.z))
    }

    /// Return the axis-specified interval according to the index.
    pub const fn axis_interval(&self, axis: Axis) -> Interval {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    /// Check if ray intersects with AABB.
    pub fn intersect(&self, r: &Ray, ray_t: Interval) -> bool {
        let mut bounds = ray_t;

        // Check intersection with three pairs of planes
        for axis in [Axis::X, Axis::Y, Axis::Z] {
            let interval = self.axis_interval(axis);
            let inv_d = 1.0 / r.dir[axis as usize];
            let mut t0 = (interval.min - r.ori[axis as usize]) * inv_d;
            let mut t1 = (interval.max - r.ori[axis as usize]) * inv_d;
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
        let delta = 1e-3;
        if self.x.size() < delta {
            self.x.extend(delta);
        }
        if self.y.size() < delta {
            self.y.extend(delta);
        }
        if self.z.size() < delta {
            self.z.extend(delta);
        }
        self
    }

    /// Get the longest axis of the aabb: 0 for x, 1 for y, 2 for z.
    pub fn longest_axis(&self) -> Axis {
        if self.x.size() >= self.y.size() && self.x.size() >= self.z.size() {
            Axis::X
        } else if self.y.size() >= self.z.size() {
            Axis::Y
        } else {
            Axis::Z
        }
    }
}
