use std::f32;

use glam::FloatExt;
use rand::{Rng, rngs::StdRng};
use rand_distr::{Distribution, UnitCircle};

use crate::{
    color::{self, Color},
    math::{Vec3, vec3::random_cosine_weight_on_hemisphere}, onb::ONB,
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

    /// Get the incident ray and the PDF according to the given normal vector and light towards view.
    /// 
    /// Useful references: 
    /// https://agraphicsguynotes.com/posts/sample_microfacet_brdf/
    pub fn scatter(&self, rng: &mut StdRng, n: Vec3, v: Vec3) -> Option<(Vec3, f32)> {
        let m2 = self.roughness * self.roughness;
        let world_onb = ONB::new(n);

        // Estimate specular contribution using Fresnel.
        let f0 = ((self.index_of_refraction - 1.0) / (self.index_of_refraction + 1.0)).powi(2);
        let f = f0.lerp(self.color.element_sum() / 3.0, self.metallic);
        // Raise the specular probability to at least 0.2.
        let f = 0.2.lerp(1.0, f);

        // Probability Integral Transform
        let beckmann = |rng: &mut StdRng| {
            // θ = arctan √(-m^2 ln U)
            let theta = (-m2 * rng.random::<f32>().ln()).sqrt().atan();
            let (sin_t, cos_t) = theta.sin_cos();
            let [x, y]: [f32; 2] = UnitCircle.sample(rng);
            let h = Vec3::new(x * sin_t, y * sin_t, cos_t);
            world_onb.transform(h)
        };

        let beckmann_pdf = |h: Vec3| {
            // p = 2 sinθ / (m^2 cos^3 θ) * e^(-tan^2(θ) / m^2)
            let cos_t = n.dot(h);
            let sin_t = (1.0-cos_t.powi(2)).sqrt();
            2.0 *sin_t / (m2*cos_t.powi(3)) * (-(sin_t/cos_t).powi(2)/m2).exp()
        };

        let l = 
        // specular
        if rng.random_bool(f as f64) {
            let h = beckmann(rng);
            -v.reflect(h)
        } 
        // diffuse
        else if !self.transparent {
            let dir = random_cosine_weight_on_hemisphere();
            world_onb.transform(dir)
        }
        // transmit
        else {
            let h = beckmann(rng);
            let cos_v = h.dot(v);
            let v_perp = v - h * cos_v;
            let l_perp = -v_perp / self.index_of_refraction;
            let sin2_l = l_perp.length_squared();
            if sin2_l > 1.0 {
                return None;
            }
            let cos_l = (1.0-sin2_l).sqrt();
            - cos_v.signum() * h * cos_l + l_perp
        };

        // Multiple Importance Sampling
        let mut pdf = 0.0;
        pdf += {
            let h = (l+v).normalize();
            let p_h = beckmann_pdf(h);
            // TODO: why abs?
            f * p_h / (4.0 * h.dot(v).abs())
        };
        pdf += 
        // diffuse component
        if !self.transparent {
            (1.0 - f) * n.dot(l) * f32::consts::FRAC_1_PI
        } 
        // transmit component
        else if n.dot(v).is_sign_positive() != n.dot(l).is_sign_positive() {
            let eta_t = if v.dot(n) > 0.0 {
                self.index_of_refraction
            } else {
                1.0 / self.index_of_refraction
            };
            let h = (l*eta_t+v).normalize();
            let h_dot_v = h.dot(v);
            let h_dot_l = h.dot(l);
            let jacobian = h_dot_v.abs() / (eta_t * h_dot_l + h_dot_v).powi(2);
            let p_h = beckmann_pdf(h);
            (1.0 - f)*p_h*jacobian
        } else {
            0.0
        };

        Some((l, pdf))
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
