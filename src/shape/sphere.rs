use crate::aabb::Aabb;
use crate::math::Point3;
use crate::math::Ray;
use crate::shape::{HitRecord, Shape};

pub struct Sphere {
    /// The center point of the sphere
    center: Point3,

    /// The radius of the sphere
    radius: f32,
}

impl Sphere {
    /// Create a sphere from center and radius
    pub fn new(center: Point3, radius: f32) -> Self {
        Sphere { center, radius }
    }

    /// Create a sphere from radius and center which is a array that has only 3 elements
    pub fn from_array(center: [f32; 3], radius: f32) -> Self {
        Sphere {
            center: Point3::from_array(center),
            radius,
        }
    }
}

impl Shape for Sphere {
    fn intersect(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
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
        rec.p = r.at(rec.t);
        // normalized normal vector
        let normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, normal);

        true
    }

    fn bounding_box(&self) -> Aabb {
        let radius_vec = Point3::splat(self.radius);
        Aabb::new(self.center - radius_vec, self.center + radius_vec)
    }
}
