use std::{f32, sync::Arc};

use crate::{
    aabb::Aabb,
    interval::Interval,
    math::random,
    math::{Ray, Vec3},
    shape::{Bounded, HitRecord, Hittable},
};

pub struct ConstantMedium {
    boundary: Arc<dyn Bounded>,
    neg_inv_density: f32,
}

impl ConstantMedium {
    pub fn new<T>(boundary: T, density: f32) -> Self
    where
        T: Bounded + 'static,
    {
        let neg_inv_density = -1.0 / density;
        Self {
            boundary: Arc::new(boundary),
            neg_inv_density,
        }
    }
}

impl Hittable for ConstantMedium {
    fn intersect(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        self.boundary.intersect(r, Interval::universe())?;
        self.boundary.intersect(
            r,
            Interval {
                min: rec1.t + 0.0001,
                max: f32::INFINITY,
            },
        )?;

        rec1.t = rec1.t.max(ray_t.min);
        rec2.t = rec2.t.min(ray_t.max);

        if rec1.t >= rec2.t {
            return None;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.dir.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let mut rec = HitRecord::default();
        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);
        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        Some(rec)
    }
}

impl Bounded for ConstantMedium {
    fn bbox(&self) -> Aabb {
        self.boundary.bbox()
    }
}
