use glam::{Mat4, Vec3A, Vec4Swizzles};
use rand::Rng;
use std::f32;

pub fn random() -> f32 {
    rand::rng().random()
}

pub fn random_in_range(min: u32, max: u32) -> u32 {
    min + (rand::rng().random::<f32>() * (max - min) as f32) as u32
}

pub type Vec3 = Vec3A;
pub type Point3 = Vec3A;

/// Vector utilities module for Vec3 operations
pub mod vec3 {
    use super::*;
    use rand::{random_range, rngs::StdRng};
    use rand_distr::{Distribution, UnitDisc};

    /// Generate a random vector with each component in [0, 1)
    #[inline]
    pub fn random_vec() -> Vec3 {
        Vec3::new(super::random(), super::random(), super::random())
    }

    /// Generate a random vector with each component in [min, max)
    #[inline]
    pub fn random_in_range(min: f32, max: f32) -> Vec3 {
        Vec3::new(
            random_range(min..max),
            random_range(min..max),
            random_range(min..max),
        )
    }

    /// Randomly generate a vector on the surface of a unit hemisphere using uniform probability
    /// density sampling.
    #[inline]
    pub fn random_on_hemisphere() -> Vec3 {
        let r1 = random();
        let r2 = random();
        let x = (2.0 * f32::consts::PI * r1).cos() * (r2 * (2.0 - r2)).sqrt();
        let y = (2.0 * f32::consts::PI * r1).sin() * (r2 * (2.0 - r2)).sqrt();
        let z = 1.0 - r2;
        Vec3::new(x, y, z)
    }

    /// Randomly generate a vector on the surface of a unit hemisphere using Malley's method.
    #[inline]
    pub fn random_cosine_weight_on_hemisphere(rng: &mut StdRng) -> Vec3 {
        let [x, y]: [f32; 2] = UnitDisc.sample(rng);
        let z = (1.0 - x * x - y * y).sqrt();
        Vec3::new(x, y, z)
    }
}

pub trait Vec3Ext {
    /// Check if the vector is close to zero in length
    fn near_zero(&self) -> bool;
}

impl Vec3Ext for Vec3 {
    #[inline]
    fn near_zero(&self) -> bool {
        self.length_squared() < f32::EPSILON
    }
}

#[derive(Default)]
/// A ray can be represented as: `A + t*B` where `A` is origin, `B` is direction, and `t` is a scalar.
/// For any given value of t, we can compute the point along the ray using the `at` method below.
pub struct Ray {
    /// The origin coordinate of ray
    pub ori: Point3,

    /// The normalized direction vector of ray
    pub dir: Vec3,

    /// The macro time to define the position of moving objects.
    /// It can be understood as the ray entering the camera at shutter time `t`.
    /// We use macro time `t` in `Ray` to distinguish different ray and micro time `t` in `HitRecord`
    /// to distinguish different point in the same ray.
    pub t: f32,
}

impl Ray {
    /// Create a ray from origin, direction and time
    pub const fn new(origin: Point3, direction: Vec3, time: f32) -> Self {
        Self {
            ori: origin,
            dir: direction,
            t: time,
        }
    }

    /// Get the point along the ray at micro time t.
    pub fn at(&self, t: f32) -> Point3 {
        self.ori + t * self.dir
    }

    pub fn apply_transform(&self, trans: &Mat4) -> Self {
        let origin = trans.mul_vec4(self.ori.extend(1.0));
        // Direction no need to translate
        let direction = trans.mul_vec4(self.dir.extend(0.0));
        Self {
            ori: origin.xyz().to_vec3a(),
            dir: direction.xyz().to_vec3a(),
            t: self.t,
        }
    }
}
