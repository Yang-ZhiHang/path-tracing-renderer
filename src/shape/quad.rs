use std::f32;

use crate::{
    aabb::Aabb,
    interval::Interval,
    math::{Point3, Ray, Vec3, random},
    shape::{Bounded, HitRecord, Hittable},
};

#[allow(non_snake_case)]
pub struct Quad {
    /// The origin of the quadrilateral plane.
    pub origin: Point3,

    /// The basis vector u defines a x-aixs of the plane.
    pub u: Vec3,

    /// The basis vector v defines a y-aixs of the plane.
    pub v: Vec3,

    /// The normalized normal vector of the quad plane.
    pub normal: Vec3,

    /// The area of the limited plane.
    pub area: f32,

    /// Plane constant in Ax+By+Cz=D
    pub D: f32,

    /// P = Q + alpha * u + beta * v
    /// alpha = w * (p x v)
    /// beta  = w * (u x p)
    /// Where:
    /// p = P - Q, w = n / (n * n)
    pub w: Vec3,

    /// The axis-aligned bounding box of sphere.
    pub aabb: Aabb,
}

#[allow(non_snake_case)]
impl Quad {
    pub fn new(origin: Point3, u: Vec3, v: Vec3) -> Self {
        let n = u.cross(v);
        let area = n.length();
        let normal = n.normalize();
        let D = normal.dot(origin);
        let w = n / (n.dot(n));

        let bbox_diagonal1 = Aabb::from_points(origin, origin + u + v);
        let bbox_diagonal2 = Aabb::from_points(origin + u, origin + v);
        let aabb = Aabb::surrounding_box(&bbox_diagonal1, &bbox_diagonal2).padding_to_minimal();

        Self {
            origin,
            u,
            v,
            normal,
            area,
            D,
            w,
            aabb,
        }
    }

    pub fn is_interior(a: f32, b: f32, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }
        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for Quad {
    fn intersect(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let denominator = self.normal.dot(r.dir);

        // Treat near-parallel rays as misses
        if denominator.abs() < f32::EPSILON {
            return None;
        }

        // Solve for the intersection parameter t
        let root = (self.D - self.normal.dot(r.ori)) / denominator;
        if !ray_t.contains(root) {
            return None;
        }

        // Determine whether this point in the area
        let p = r.at(root) - self.origin;
        let alpha = self.w.dot(p.cross(self.v));
        let beta = self.w.dot(self.u.cross(p));
        let mut rec = HitRecord::default();
        if !Self::is_interior(alpha, beta, &mut rec) {
            return None;
        }

        // Set intersection record
        rec.t = root;
        rec.p = r.at(root);
        rec.set_face_normal(r, self.normal);

        Some(rec)
    }

    /// Return the PDF of the hittable shape.
    fn pdf(&self, r_out: &Ray) -> f32 {
        if let Some(rec) = self.intersect(r_out, Interval::new(1e-3, f32::INFINITY)) {
            let distance_squared = (rec.p - r_out.ori).length_squared();
            let cos = r_out.dir.normalize().dot(self.normal).abs();
            return distance_squared / (self.area * cos);
        }
        1.0 / (2.0 * f32::consts::PI)
    }

    /// Return a random ray from given point to the hittable shape.
    fn random(&self, origin: Vec3) -> Vec3 {
        let p = self.origin + random() * self.u + random() * self.v;
        p - origin
    }

    fn sample(&self, target: Point3, _rng: &mut rand::prelude::StdRng) -> (Point3, Vec3, f32) {
        let alpha = random();
        let beta = random();
        let p = self.origin + alpha * self.u + beta * self.v;
        let normal = self.normal;
        let cos_t = normal.dot(target - p).abs();
        let surface_area = self.area * cos_t;
        let pdf = 1.0 / surface_area;
        (p, normal, pdf)
    }
}

impl Bounded for Quad {
    fn bbox(&self) -> Aabb {
        self.aabb
    }
}
