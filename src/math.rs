use crate::common::random;
use glam::Vec3A;
use rand::random_range;
use std::f32::EPSILON;

pub type Vec3 = Vec3A;

pub trait Vec3Ext {
    /// Generate a random vector with each component in [0, 1)
    fn random() -> Vec3;

    /// Generate a random vector with each component in [min, max)
    fn random_range(min: f32, max: f32) -> Vec3;

    /// Randomly Generate a vector in a unit sphere which length <= 1.0
    fn random_in_unit_sphere() -> Vec3;

    /// Randomly Generate a vector on the surface of a unit sphere which length == 1.0
    fn random_unit_vector() -> Vec3;

    /// Randomly Generate a vector in a unit disk which length <= 1.0
    fn random_in_unit_disk() -> Vec3;

    /// Check if the vector is close to zero in length
    fn near_zero(&self) -> bool;
}

impl Vec3Ext for Vec3 {
    #[inline]
    fn random() -> Vec3 {
        Vec3::new(random(), random(), random())
    }

    #[inline]
    fn random_range(min: f32, max: f32) -> Vec3 {
        Vec3::new(
            random_range(min..max),
            random_range(min..max),
            random_range(min..max),
        )
    }

    #[inline]
    fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Self::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    #[inline]
    fn random_unit_vector() -> Vec3 {
        Self::random_in_unit_sphere().normalize()
    }

    #[inline]
    fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(random_range(-1.0..1.0), random_range(-1.0..1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    #[inline]
    fn near_zero(&self) -> bool {
        self.length_squared() < EPSILON
    }
}

pub type Point3 = Vec3A;
pub type Color = Vec3A;

/// Use rgb instead of xyz in type Color
pub trait ColorExt {
    fn rgb(r: f32, g: f32, b: f32) -> Self;
    fn black() -> Self;
    fn r(&self) -> f32;
    fn g(&self) -> f32;
    fn b(&self) -> f32;
}

impl ColorExt for Color {
    #[inline]
    fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r, g, b)
    }

    #[inline]
    fn black() -> Self {
        Self::ZERO
    }

    #[inline]
    fn r(&self) -> f32 {
        self.x
    }

    #[inline]
    fn g(&self) -> f32 {
        self.y
    }

    #[inline]
    fn b(&self) -> f32 {
        self.z
    }
}

#[derive(Default)]
/// A ray can be represented as: A + t*B where A is origin, B is direction, and t is a scalar
/// that indicates how long the ray has traveled. For any given value of t, we can compute the
/// point along the ray using the `at` method below.
pub struct Ray {
    /// The origin coordinate of ray
    pub origin: Point3,

    /// The direction vector of ray
    pub direction: Vec3,

    /// The time to define the position of moving objects. It can be understood as the ray
    /// entering the camera at time t.
    pub t: f32,
}

impl Ray {
    /// Create a ray from origin, direction and time
    pub fn new(origin: Point3, direction: Vec3, time: f32) -> Self {
        Ray {
            origin,
            direction,
            t: time,
        }
    }

    /// Get the point along the ray at time t.
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}
