use std::f32::consts::PI;

use crate::aabb::Aabb;
use crate::interval::Interval;
use crate::math::Point3;
use crate::math::Ray;
use crate::math::Vec3;
use crate::shape::Hittable;
use crate::shape::{Bounded, HitRecord};

pub struct Sphere {
    /// The center point of the sphere.
    center: Ray,

    /// The radius of the sphere.
    radius: f32,

    /// The axis-aligned bounding box of sphere.
    aabb: Aabb,
}

impl Sphere {
    /// Create a sphere from center and radius. If center_to is provided, the sphere moves linearly
    /// from center_from to center_to as time t goes from 0.0 to 1.0.
    pub fn new(center_from: Point3, center_to: Option<Point3>, radius: f32) -> Self {
        // Use absolute radius so negative-radius spheres still produce a valid box
        let r = radius.abs();
        let radius_vec = Point3::splat(r);
        let (center_direction, aabb) = match center_to {
            Some(ct) => {
                let box_from = Aabb::from_points(center_from - radius_vec, center_from + radius_vec);
                let box_to = Aabb::from_points(ct - radius_vec, ct + radius_vec);
                (ct - center_from, Aabb::surrounding_box(&box_from, &box_to))
            }
            None => (
                Vec3::ZERO,
                Aabb::from_points(center_from - radius_vec, center_from + radius_vec),
            ),
        };
        Sphere {
            center: Ray::new(center_from, center_direction, 0.0),
            radius,
            aabb,
        }
    }

    pub fn get_sphere_uv(p: Point3) -> (f32, f32) {
        // Normalize to make UV mapping independent of radius length
        let p = p.normalize();
        let phi = (-p.z).atan2(p.x) + PI;
        let theta = (-p.y).acos();
        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
    }
}

impl Hittable for Sphere {
    fn intersect(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // oc = A - C
        let current_center = self.center.at(r.t);
        let oc = r.origin - current_center;
        let a = r.direction.length_squared();
        let h = r.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_d = discriminant.sqrt();
        let mut root = (-h - sqrt_d) / a; // Find the nearest root, start with (-b-sqrt_d)
        if !ray_t.contains(root) {
            root = (-h + sqrt_d) / a;
            if !ray_t.contains(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(root);
        // If radius is negative, the normal is inverted. Application: hollow glass sphere.
        let normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, normal);
        (rec.u, rec.v) = Self::get_sphere_uv(normal);

        true
    }
}

impl Bounded for Sphere {
    fn bbox(&self) -> Aabb {
        self.aabb
    }
}
