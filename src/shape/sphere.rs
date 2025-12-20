use crate::math::Point3;
use crate::math::Ray;
use crate::shape::{HitRecord, Hitable};

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: [f32; 3], radius: f32) -> Self {
        Sphere {
            center: Point3::from_array(center),
            radius,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        // dst = A - C
        let dst = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = r.direction().dot(dst);
        let c = dst.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_d = f32::sqrt(discriminant);

        // Find the nearest root, start with (-b - sqrt_d)
        let mut root = (-half_b - sqrt_d) / a;
        if root <= t_min || root >= t_max {
            root = (-half_b + sqrt_d) / a;
            if root <= t_min || root >= t_max {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(root);
        rec.normal = (rec.p - self.center) / self.radius; // unit normal vector

        true
    }
}
