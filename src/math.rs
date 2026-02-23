use glam::{DMat4, DVec3, Vec4Swizzles};
use rand::Rng;
use std::f64;

pub type DPoint3 = DVec3;

pub fn random() -> f64 {
    rand::rng().random()
}

pub fn random_in_range(min: u32, max: u32) -> u32 {
    min + (rand::rng().random::<f64>() * (max - min) as f64) as u32
}

/// Vector utilities module for Vec3 operations
pub mod vec {
    use super::*;
    use rand::{random_range, rngs::StdRng};
    use rand_distr::{Distribution, UnitDisc};

    /// Generate a random vector with each component in [0, 1)
    #[inline]
    pub fn random_vec() -> DVec3 {
        DVec3::new(super::random(), super::random(), super::random())
    }

    /// Generate a random vector with each component in [min, max)
    #[inline]
    pub fn random_in_range(min: f64, max: f64) -> DVec3 {
        DVec3::new(
            random_range(min..max),
            random_range(min..max),
            random_range(min..max),
        )
    }

    /// Randomly generate a vector on the surface of a unit hemisphere using uniform probability
    /// density sampling.
    #[inline]
    pub fn random_on_hemisphere() -> DVec3 {
        let r1 = random();
        let r2 = random();
        let x = (2.0 * f64::consts::PI * r1).cos() * (r2 * (2.0 - r2)).sqrt();
        let y = (2.0 * f64::consts::PI * r1).sin() * (r2 * (2.0 - r2)).sqrt();
        let z = 1.0 - r2;
        DVec3::new(x, y, z)
    }

    /// Randomly generate a vector on the surface of a unit hemisphere using Malley's method.
    #[inline]
    pub fn random_cosine_weight_on_hemisphere(rng: &mut StdRng) -> DVec3 {
        let [x, y]: [f64; 2] = UnitDisc.sample(rng);
        let z = (1.0 - x * x - y * y).sqrt();
        DVec3::new(x, y, z)
    }
}

#[derive(Default)]
/// A ray can be represented as: `A + t*B` where `A` is origin, `B` is direction, and `t` is a scalar.
/// For any given value of t, we can compute the point along the ray using the `at` method below.
pub struct Ray {
    /// The origin coordinate of ray
    pub ori: DPoint3,

    /// The normalized direction vector of ray
    pub dir: DVec3,

    /// The macro time to define the position of moving objects.
    /// It can be understood as the ray entering the camera at shutter time `t`.
    /// We use macro time `t` in `Ray` to distinguish different ray and micro time `t` in `HitRecord`
    /// to distinguish different point in the same ray.
    pub t: f64,
}

impl Ray {
    /// Create a ray from origin, direction and time
    pub const fn new(origin: DPoint3, direction: DVec3, time: f64) -> Self {
        Self {
            ori: origin,
            dir: direction,
            t: time,
        }
    }

    /// Get the point along the ray at micro time t.
    pub fn at(&self, t: f64) -> DPoint3 {
        self.ori + t * self.dir
    }

    pub fn apply_transform(&self, trans: &DMat4) -> Self {
        let origin = trans.mul_vec4(self.ori.extend(1.0));
        // Direction no need to translate
        let direction = trans.mul_vec4(self.dir.extend(0.0));
        Self {
            ori: origin.xyz(),
            dir: direction.xyz(),
            t: self.t,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}
