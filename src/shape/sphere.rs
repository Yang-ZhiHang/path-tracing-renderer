use std::f32;
use std::f32::consts::PI;

use rand::rngs::StdRng;

use crate::aabb::Aabb;
use crate::interval::Interval;
use crate::math::Point3;
use crate::math::Ray;
use crate::math::Vec3;
use crate::math::vec3::random_cosine_weight_on_hemisphere;
use crate::onb::ONB;
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
        let (center_direction, aabb) = center_to.map_or_else(
            || {
                (
                    Vec3::ZERO,
                    Aabb::from_points(center_from - radius_vec, center_from + radius_vec),
                )
            },
            |ct| {
                let box_from =
                    Aabb::from_points(center_from - radius_vec, center_from + radius_vec);
                let box_to = Aabb::from_points(ct - radius_vec, ct + radius_vec);
                (ct - center_from, Aabb::surrounding_box(&box_from, &box_to))
            },
        );
        Self {
            center: Ray::new(center_from, center_direction, 0.0),
            radius,
            aabb,
        }
    }

    /// Transform 3D sphere coordinates into plane coordinates using polar angle and azimuth angle.
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
    fn intersect(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let current_center = self.center.at(r.t);
        let oc = r.ori - current_center;
        let a = r.dir.length_squared();
        let frac_b_2 = r.dir.dot(oc);
        let c = self.radius.mul_add(-self.radius, oc.length_squared());
        let discriminant = frac_b_2.mul_add(frac_b_2, -a * c);
        if discriminant.is_sign_negative() {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let mut root = (-frac_b_2 - sqrt_d) / a; // Find the nearest root
        if !ray_t.contains(root) {
            root = (-frac_b_2 + sqrt_d) / a;
            if !ray_t.contains(root) {
                return None;
            }
        }

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            ..Default::default()
        };
        // If radius is negative, the normal is inverted. Application: hollow glass sphere.
        let normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, normal);
        (rec.u, rec.v) = Self::get_sphere_uv(normal);

        Some(rec)
    }

    /// Get a random point from the sphere and also return the vector from the random point to `target` and the PDF based on MIS.
    fn sample(&self, target: Point3, rng: &mut StdRng) -> (Point3, Vec3, f32) {
        let p = random_cosine_weight_on_hemisphere(rng);
        let n = (target - self.center.ori).normalize();
        let world_onb = ONB::new(n);
        let world_p = world_onb.transform(p) * self.radius + self.center.ori;
        (
            world_p,
            n,
            p.z * f32::consts::FRAC_1_PI / (self.radius * self.radius),
        ) // p.z = cosθ
    }
}

impl Bounded for Sphere {
    fn bbox(&self) -> Aabb {
        self.aabb
    }
}
