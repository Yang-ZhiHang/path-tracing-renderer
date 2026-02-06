use crate::{
    aabb::Aabb,
    interval::Interval,
    math::{Point3, Ray, Vec3},
    shape::{Bounded, HitRecord, Hittable},
};

pub struct Cube {
    /// The point which is closer to the camera.
    p_min: Point3,

    /// The point which is further to the camera.
    p_max: Point3,

    /// The axis-aligned bounding box of sphere.
    aabb: Aabb,
}

impl Cube {
    pub fn new(p1: Point3, p2: Point3) -> Self {
        let (p_min, p_max) = if p1.z < p2.z { (p1, p2) } else { (p2, p1) };
        let aabb = Aabb::new(
            Interval::new(p_min.x, p_max.x),
            Interval::new(p_min.y, p_max.y),
            Interval::new(p_min.z, p_max.z),
        );
        Self { p_min, p_max, aabb }
    }
}

impl Hittable for Cube {
    fn intersect(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut t_min = ray_t.min;
        let mut t_max = ray_t.max;

        // Check intersection with three pairs of planes
        for axis in 0..3 {
            let inv_d = 1.0 / r.dir[axis];
            let t0 = (self.p_min[axis] - r.ori[axis]) * inv_d;
            let t1 = (self.p_max[axis] - r.ori[axis]) * inv_d;

            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };

            t_min = t_min.max(t0);
            t_max = t_max.min(t1);

            if t_max < t_min {
                return None;
            }
        }

        // Find the hit point
        let t = if t_min >= ray_t.min { t_min } else { t_max };
        if t < ray_t.min || t > ray_t.max {
            return None;
        }

        let mut rec = HitRecord {
            t,
            p: r.at(t),
            ..Default::default()
        };

        // Calculate normal vector based on which face was hit
        let epsilon = 1e-4;
        let normal = if (rec.p.x - self.p_min.x).abs() < epsilon {
            Vec3::new(-1.0, 0.0, 0.0)
        } else if (rec.p.x - self.p_max.x).abs() < epsilon {
            Vec3::new(1.0, 0.0, 0.0)
        } else if (rec.p.y - self.p_min.y).abs() < epsilon {
            Vec3::new(0.0, -1.0, 0.0)
        } else if (rec.p.y - self.p_max.y).abs() < epsilon {
            Vec3::new(0.0, 1.0, 0.0)
        } else if (rec.p.z - self.p_min.z).abs() < epsilon {
            Vec3::new(0.0, 0.0, -1.0)
        } else {
            Vec3::new(0.0, 0.0, 1.0)
        };

        rec.normal = normal;
        Some(rec)
    }
}

impl Bounded for Cube {
    fn bbox(&self) -> Aabb {
        self.aabb
    }
}
