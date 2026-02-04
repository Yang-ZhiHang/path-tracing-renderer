use std::f32;

use crate::{
    color::{self, Color},
    math::Vec3,
};

pub struct Material {
    pub color: Color,
    pub roughness: f32,
    pub metallic: f32,
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
            metallic: 0.0,
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
            metallic: 0.0,
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
            metallic: 0.0,
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
            metallic: 1.0,
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
            metallic: 0.0,
            index_of_refraction: 1.0,
            emittance,
            transparent: false,
        }
    }
}

impl Material {
    /// Bi-direction Scatter Distribution Function. Including BRDF and BTDF.
    ///
    /// - `l`: The direction from the intersection point towards light.
    /// - `v`: The direction from the intersection point towards view.
    /// - `n`: The normal vector of the surface of the intersection point.
    ///
    /// Returning the function describes the distribution of scattering.
    pub fn bsdf(&self, l: Vec3, v: Vec3, n: Vec3) -> Vec3 {
        let n_dot_l = n.dot(l);
        let n_dot_v = n.dot(v);
        let l_outside = n_dot_l.is_sign_positive();
        let v_outside = n_dot_v.is_sign_positive();
        let h = (l + v).normalize();
        let n_dot_h = n.dot(h);
        let v_dot_h = v.dot(h);
        let nh2 = n_dot_h.powi(2);

        // d: microfacet distribution function
        // D = exp(((n • h)^2 - 1) / (m^2 (n • h)^2)) / (π m^2 (n • h)^4)
        // TODO: try different formula: https://zhuanlan.zhihu.com/p/152226698
        let m2 = self.roughness * self.roughness;
        let d = ((nh2 - 1.0) / (m2 * nh2)).exp() / (f32::consts::PI * m2 * nh2 * nh2);

        // f: fresnel, schlick's approximation
        // F = F0 + (1 - F0)(1 - v • h)^5
        let f = if !v_outside && (1.0 - n_dot_v * n_dot_v).sqrt() * self.index_of_refraction > 1.0 {
            Vec3::splat(1.0)
        } else {
            let f0 = ((self.index_of_refraction - 1.0) / (self.index_of_refraction + 1.0)).powi(2);
            let f0 = Vec3::splat(f0).lerp(self.color, self.metallic);
            (1.0 - f0).mul_add(Vec3::splat((1.0 - n_dot_v).powi(5)), f0)
        };

        // g: geometry function, microfacet shadowing
        // G = min(1, 2(n • h)(n • v)/(v • h), 2(n • h)(n • l)/(v • h))
        let g = (n_dot_h * n_dot_v).min(n_dot_v * n_dot_h);
        let g = 2.0 * g / v_dot_h;
        let g = g.min(1.0);

        if l_outside == v_outside {
            // BRDF
            // Cook-Torrance = DFG / (4(n • l)(n • v))
            // Lambert = (1 - F) * c / π
            let specular = d * f * g / (4.0 * n_dot_v * n_dot_l);
            if self.transparent {
                specular
            } else {
                let diffuse = (1.0 - f) * self.color / f32::consts::PI;
                specular + diffuse
            }
        } else {
            // Ratio of refractive indices, η_i / η_o
            let eta_t = if v_outside {
                self.index_of_refraction
            } else {
                1.0 / self.index_of_refraction
            };
            let l_dot_h = l.dot(h);

            // BTDF
            // Cook-Torrance = |l • h|/|n • l| * |v • h|/|n • v| * (1 - F)DG / (η_i / η_o * (h • l) + (h • v))^2
            let btdf = (l_dot_h * v_dot_h / (n_dot_l * n_dot_v)).abs()
                * ((1.0 - f) * d * g / (eta_t * l_dot_h + v_dot_h).powi(2));
            btdf * self.color
        }
    }

    pub fn scatter(&self) -> Option<(Vec3, f32)> {
        todo!()
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
