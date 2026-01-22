use std::f32;

use crate::{
    color::{self, Color},
    math::{Point3, Ray},
    shape::HitRecord,
};

pub mod dieletrics;
pub mod isotropic;
pub mod lambertian;
pub mod light;
pub mod metal;

pub trait Material: Send + Sync {
    /// Get the attenuation color and scattered ray to be able to compute the scattered color.
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    /// Get the emitted color of the material at the given uv coordinate and position. No emit by default (return `color::Black`).
    fn emit(&self, _u: f32, _v: f32, _p: Point3) -> Color {
        color::BLACK
    }

    /// Use Schlick's approximation for reflectance.
    fn reflectance(&self, cos: f32, eta: f32) -> f32 {
        let mut r0 = (1.0 - eta) / (1.0 + eta);
        r0 = r0 * r0;
        (1.0 - r0).mul_add((1.0 - cos).powi(5), r0)
    }

    /// Return the BRDF of a material. Default to uniform sampling in hemisphere.
    fn brdf(&self, _r_in: &Ray, _r_out: &Ray, _rec: &HitRecord) -> f32 {
        1.0 / (2.0 * f32::consts::PI)
    }

    /// Return the probability density function of a material. Default to uniform sampling in hemisphere.
    fn pdf_value(&self) -> f32 {
        1.0 / (2.0 * f32::consts::PI)
    }
}
