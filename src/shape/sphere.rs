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
        // oc = A - C
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let h = r.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_d = discriminant.sqrt();
        let mut root = (-h - sqrt_d) / a; // Find the nearest root, start with (-b-sqrt_d)
        if root <= t_min || root >= t_max {
            root = (-h + sqrt_d) / a;
            if root <= t_min || root >= t_max {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(root);
        let normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, normal);

        true
    }
}
