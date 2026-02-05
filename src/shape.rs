use std::{f32, sync::Arc};

use glam::{Mat3A, Mat4, Vec4, Vec4Swizzles};
use rand::rngs::StdRng;

use crate::{
    aabb::Aabb,
    interval::Interval,
    material::Material,
    math::{Point3, Ray, Vec3},
};

pub mod constant_medium;
pub mod cube;
pub mod quad;
pub mod sphere;

pub trait Hittable: Send + Sync {
    /// Used for `HitRecord` of incident ray.
    fn intersect(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;

    /// Return a random point, normal and the pdf.
    /// The function is a combination of `pdf` and `random` in Ray Tracing Series 3.
    fn sample(&self, _target: Point3, _rng: &mut StdRng) -> (Point3, Vec3, f32) {
        (
            Point3::splat(1.0),
            Vec3::splat(1.0),
            1.0 / (2.0 * f32::consts::PI),
        )
    }
}

pub trait Bounded: Hittable {
    /// The bounding box of the shape.
    fn bbox(&self) -> Aabb;
}

#[derive(Default, Clone)]
pub struct HitRecord {
    /// The 3d coordinations of intersection point.
    pub p: Point3,

    /// Time which can be used to compute point along the ray through the formula
    /// p = origin + t * direction. This attribute is more microscopic than the time of
    /// the `Ray` structure.
    pub t: f32,

    /// The 3d coordinations of the normal vector in the intersection surface towards
    /// the incident ray.
    pub normal: Vec3,

    /// The flag to determine whether the normal vector towards you. e.g. if the radius is
    /// negative, then the normal vector is inverted.
    pub front_face: bool,

    /// The material of intersect object.
    pub material: Option<Arc<Material>>,

    /// The coordinates of the object surface mapping to the texture map
    pub u: f32,
    pub v: f32,
}

impl HitRecord {
    /// Set the normal vector of intersections surface which face to the incident ray.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }

    pub fn material(&self) -> &Material {
        self.material.as_ref().unwrap()
    }
}

/// A Object that has been composed with a transformation.
pub struct Transformed<T> {
    /// The hittable shape that need to transforme.
    shape: T,

    /// The transformation matrix to transform object.
    transform: Mat4,

    #[allow(dead_code)]
    /// The transformation which extract from `transform` and not contains translate part.
    linear: Mat3A,

    /// The inverse of `transform` which use to transform the incident ray.
    inverse_transform: Mat4,

    /// The inverse and transpose of `transform` which use to rectify normal vector.
    normal_transform: Mat3A,
}

impl<T> Transformed<T> {
    pub fn new(shape: T, transform: Mat4) -> Self {
        let linear = Mat3A::from_mat4(transform);
        let inverse_transform = transform.inverse();
        let normal_transform = linear.inverse().transpose();
        Self {
            shape,
            transform,
            linear,
            inverse_transform,
            normal_transform,
        }
    }
}

impl<T: Hittable> Hittable for Transformed<T> {
    fn intersect(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let ray_trans = r.apply_transform(&self.inverse_transform);
        match self.shape.intersect(&ray_trans, ray_t) {
            None => None,
            Some(mut rec) => {
                // Transform intersection point back to world space
                let p_world = self.transform * rec.p.extend(1.0);
                rec.p = p_world.xyz().to_vec3a();

                // Fix normal vector by multiplying by M^-T
                rec.normal = self.normal_transform.mul_vec3a(rec.normal).normalize();

                // Check face normal against the original ray (not transformed ray)
                rec.set_face_normal(r, rec.normal);
                Some(rec)
            }
        }
    }
}

impl<T: Bounded> Bounded for Transformed<T> {
    fn bbox(&self) -> Aabb {
        let Aabb { x, y, z } = self.shape.bbox();

        // Transform all 8 corners and find min/max
        let corners = [
            (x.min, y.min, z.min),
            (x.min, y.min, z.max),
            (x.min, y.max, z.min),
            (x.min, y.max, z.max),
            (x.max, y.min, z.min),
            (x.max, y.min, z.max),
            (x.max, y.max, z.min),
            (x.max, y.max, z.max),
        ];

        let transformed: Vec<_> = corners
            .iter()
            .map(|&(x, y, z)| {
                let p = (self.transform * Vec4::new(x, y, z, 1.0)).xyz();
                (p.x, p.y, p.z)
            })
            .collect();

        let min_x = transformed
            .iter()
            .map(|p| p.0)
            .fold(f32::INFINITY, f32::min);
        let max_x = transformed
            .iter()
            .map(|p| p.0)
            .fold(f32::NEG_INFINITY, f32::max);
        let min_y = transformed
            .iter()
            .map(|p| p.1)
            .fold(f32::INFINITY, f32::min);
        let max_y = transformed
            .iter()
            .map(|p| p.1)
            .fold(f32::NEG_INFINITY, f32::max);
        let min_z = transformed
            .iter()
            .map(|p| p.2)
            .fold(f32::INFINITY, f32::min);
        let max_z = transformed
            .iter()
            .map(|p| p.2)
            .fold(f32::NEG_INFINITY, f32::max);

        Aabb {
            x: Interval::new(min_x, max_x),
            y: Interval::new(min_y, max_y),
            z: Interval::new(min_z, max_z),
        }
    }
}

pub trait Transformable<T> {
    /// Translate the shape from vector.
    fn translate(self, v: Vec3) -> Transformed<T>;

    /// Rotate the shape from a specified angle in radians
    fn rotate(self, axis: Vec3, angle: f32) -> Transformed<T>;

    /// Rotate the shape in y-axis from a specified angle in radians
    fn rotate_y(self, angle: f32) -> Transformed<T>;
}

impl<T: Hittable> Transformable<T> for T {
    fn translate(self, v: Vec3) -> Transformed<T> {
        Transformed::new(self, Mat4::from_translation(v.into()))
    }
    fn rotate(self, axis: Vec3, angle: f32) -> Transformed<T> {
        Transformed::new(self, Mat4::from_axis_angle(axis.into(), angle))
    }
    fn rotate_y(self, angle: f32) -> Transformed<T> {
        Transformed::new(self, Mat4::from_rotation_y(angle))
    }
}
