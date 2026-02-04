use std::f32;

use crate::{
    color::{self, Color}
};

pub struct Material {
    pub color: Color,
    pub roughness: f32,
    pub index_of_refraction: f32,
    pub emittance: f32,
    pub transparent: bool,
}

impl Material {
    /// Specular reflection material with specified color.
    pub fn specular(color: Color) -> Self {
        Self {
            color,
            roughness: 0.0,
            index_of_refraction: 1.0,
            emittance: 0.0,
            transparent: false,
        }
    }

    /// Specular diffuse material with specified color.
    pub fn diffuse(color: Color) -> Self {
        Self {
            color,
            roughness: 1.0,
            index_of_refraction: 1.0,
            emittance: 0.0,
            transparent: false,
        }
    }

    /// Transparent glass material with specified roughness and index of refraction.
    pub fn clear(roughness: f32, eta: f32) -> Self {
        Self {
            color: color::WHITE,
            roughness,
            index_of_refraction: eta,
            emittance: 0.0,
            transparent: true,
        }
    }

    /// Metal material with specified color and roughness.
    pub fn metallic(color: Color, roughness: f32) -> Self {
        Self {
            color,
            roughness,
            index_of_refraction: 1.0,
            emittance: 0.0,
            transparent: false,
        }
    }

    /// Light material with specified color and emittance.
    pub fn light(color: Color, emittance: f32) -> Self {
        Self {
            color,
            roughness: 0.0,
            index_of_refraction: 1.0,
            emittance,
            transparent: false,
        }
    }
}

// pub trait Material: Send + Sync {
//     /// Get the attenuation color and scattered ray to be able to compute the scattered color.
//     fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
//         None
//     }

//     /// Get the emitted color of the material at the given uv coordinate and position. No emit by default (return `color::Black`).
//     fn emit(&self, _u: f32, _v: f32, _p: Point3) -> Color {
//         color::BLACK
//     }

//     /// Use Schlick's approximation for reflectance.
//     fn reflectance(&self, cos: f32, eta: f32) -> f32 {
//         let mut r0 = (1.0 - eta) / (1.0 + eta);
//         r0 = r0 * r0;
//         (1.0 - r0).mul_add((1.0 - cos).powi(5), r0)
//     }

//     /// Return the BRDF of a material. Default to uniform sampling in hemisphere.
//     fn brdf(&self, _r_in: &Ray, _r_out: &Ray, _rec: &HitRecord) -> f32 {
//         1.0 / (2.0 * f32::consts::PI)
//     }

//     /// Return the probability density function of a material. Default to uniform sampling in hemisphere.
//     fn pdf_value(&self) -> f32 {
//         1.0 / (2.0 * f32::consts::PI)
//     }
// }
