use std::f32;

use glam::FloatExt;
use rand::{Rng, rngs::StdRng};
use rand_distr::{Distribution, UnitCircle};

use crate::{
    color::{self, Color},
    math::{Vec3, vec3::random_cosine_weight_on_hemisphere},
    onb::ONB,
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
    pub fn specular(color: Color, roughness: f32) -> Self {
        Self {
            color,
            roughness: roughness.clamp(1e-2, 1.0),
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
    pub fn clear(roughness: f32, index_of_refraction: f32) -> Self {
        Self {
            color: color::WHITE,
            roughness: roughness.clamp(1e-2, 1.0),
            metallic: 0.0,
            index_of_refraction,
            emittance: 0.0,
            transparent: true,
        }
    }

    /// Metal material with specified color and roughness.
    pub fn metallic(color: Color, roughness: f32) -> Self {
        Self {
            color,
            roughness: roughness.clamp(1e-2, 1.0),
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
            roughness: 1.0,
            metallic: 0.0,
            index_of_refraction: 1.0,
            emittance,
            transparent: false,
        }
    }

    /// Colored transparent material
    pub fn transparent(color: Color, index_of_refraction: f32, roughness: f32) -> Self {
        Self {
            color,
            index_of_refraction,
            roughness: roughness.clamp(1e-2, 1.0),
            metallic: 0.0,
            emittance: 0.0,
            transparent: true,
        }
    }
}

impl Material {
    /// Normal Distribution Function.
    /// Here, we use Beckmann distribution to model the microfacet distribution.
    /// References:
    /// https://zhuanlan.zhihu.com/p/611622351
    pub fn beckmann(&self, nh: f32) -> f32 {
        // D = exp(-tanθ_h / m^2) / (π m^2 (n • h)^4)
        // Clamp roughness to avoid division by zero.
        let m2 = self.roughness * self.roughness;
        let nh2 = nh * nh;
        let tan2_t = (1.0 - nh2) / nh2;
        (-tan2_t / m2).exp() / (f32::consts::PI * m2 * nh2 * nh2)
    }

    /// Normal Distribution Function.
    /// Here, we use GGX (Trowbridge-Reitz) distribution to model the microfacet distribution.
    /// References:
    /// https://zhuanlan.zhihu.com/p/611622351
    pub fn ggx(&self, nh: f32) -> f32 {
        let m2 = self.roughness * self.roughness;
        let nh2 = nh * nh;
        m2 * f32::consts::FRAC_1_PI / ((nh2 * (m2 - 1.0) + 1.0).powi(2))
    }

    /// Fresnel function using Schlick's approximation.
    /// References:
    /// https://zhuanlan.zhihu.com/p/152226698
    pub fn fresnel_schlick(&self, h_dot_v: f32) -> Vec3 {
        // F = F0 + (1 - F0)(1 - v • h)^5
        let f0 = ((self.index_of_refraction - 1.0) / (self.index_of_refraction + 1.0)).powi(2);

        let f0 = Vec3::splat(f0).lerp(self.color, self.metallic);
        (1.0 - f0).mul_add(Vec3::splat((1.0 - h_dot_v).powi(5)), f0)
    }

    /// Geometry function.
    /// It seems that geometric functions need to be selected based on the normal distribution function.
    /// Here, we temporarily use Smith's method with Schlick-GGX approximation.
    /// References:
    /// https://zhuanlan.zhihu.com/p/152226698
    pub fn gf(&self, n: Vec3, l: Vec3, v: Vec3) -> f32 {
        let k = (self.roughness + 1.0).powi(2) / 8.0;
        let g = |v: Vec3| {
            let n_dot_v = n.dot(v).abs();
            n_dot_v / (n_dot_v * (1.0 - k) + k)
        };
        g(l) * g(v)
    }

    /// Bi-direction Scatter Distribution Function. Including BRDF and BTDF.
    ///
    /// - `l`: The direction from the intersection point towards light.
    /// - `v`: The direction from the intersection point towards view.
    /// - `n`: The normal vector of the surface of the intersection point.
    ///
    /// Returning the function describes the distribution of scattering.
    pub fn bsdf(&self, l: Vec3, v: Vec3, n: Vec3, front_face: bool) -> Vec3 {
        let n_dot_l = n.dot(l);
        let n_dot_v = n.dot(v);
        let l_outside = n_dot_l.is_sign_positive();
        let v_outside = n_dot_v.is_sign_positive();

        // g: geometry function, microfacet shadowing
        // g depends on n, l, v only (and roughness), not on microfacet h.
        let g = self.gf(n, l, v);

        if l_outside == v_outside {
            // Reflection

            // For reflection, h is the half vector between l and v
            let h = (l + v).normalize();
            let n_dot_h = n.dot(h);
            let h_dot_v = v.dot(h);

            // normal distribution function
            let d = self.ggx(n_dot_h);

            // fresnel, schlick's approximation
            let f = if !l_outside
                && (1.0 - n_dot_v * n_dot_v).sqrt() * self.index_of_refraction > 1.0
            {
                Vec3::splat(1.0)
            } else {
                self.fresnel_schlick(h_dot_v)
            };

            // BRDF
            // Cook-Torrance = DFG / (4(n • l)(n • v))
            // Lambert = (1 - F) * c / π
            let specular = d * f * g / (4.0 * n_dot_v * n_dot_l);
            if self.transparent {
                specular
            } else {
                let diffuse = (1.0 - f) * self.color * f32::consts::FRAC_1_PI;
                specular + diffuse
            }
        } else {
            // Transmission

            // Ratio of refractive indices, η_i / η_o
            let eta_t = if front_face {
                self.index_of_refraction
            } else {
                1.0 / self.index_of_refraction
            };

            // For refraction, h is -(η_i * l + η_o * v).normalize()
            let h = -(l * eta_t + v).normalize();

            let n_dot_h = n.dot(h);
            let h_dot_v = v.dot(h);
            let h_dot_l = l.dot(h);

            // normal distribution function with the correct transmission h
            let d = self.ggx(n_dot_h);

            // fresnel, schlick's approximation
            let f = self.fresnel_schlick(h_dot_v.abs());

            // BTDF
            // Cook-Torrance = |l • h|/|n • l| * |v • h|/|n • v| * (1 - F)DG / (η_i / η_o * (h • l) + (h • v))^2
            let btdf = (h_dot_l * h_dot_v / (n_dot_l * n_dot_v)).abs()
                * ((1.0 - f) * d * g / (eta_t * h_dot_l + h_dot_v).powi(2));
            btdf * self.color
        }
    }

    /// Get the incident ray and the PDF according to the given normal vector and light towards view.
    /// References:
    /// https://agraphicsguynotes.com/posts/sample_microfacet_brdf/
    pub fn scatter(
        &self,
        rng: &mut StdRng,
        n: Vec3,
        v: Vec3,
        front_face: bool,
    ) -> Option<(Vec3, f32)> {
        let m2 = self.roughness * self.roughness;
        let world_onb = ONB::new(n);
        let eta_t = if front_face {
            self.index_of_refraction
        } else {
            1.0 / self.index_of_refraction
        };

        // Estimate specular contribution using Fresnel.
        let f0 = ((self.index_of_refraction - 1.0) / (self.index_of_refraction + 1.0)).powi(2);
        let f = f0.lerp(self.color.element_sum() / 3.0, self.metallic);

        // Raise the specular probability to at least 0.2, but only if there is a specular component.
        // If F0 is closely 0 and not metallic, we shouldn't force specular sampling.
        let f = if f > 1e-3 { 0.2.lerp(1.0, f) } else { f };

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
            // p = 1 / (π m^2 cos^3 θ) * e^(-tan^2(θ) / m^2)
            let cos_t = n.dot(h).abs();
            let sin_t = (1.0 - cos_t.powi(2)).sqrt();
            (f32::consts::PI * m2 * cos_t.powi(3)).recip() * (-(sin_t / cos_t).powi(2) / m2).exp()
        };

        let l = if rng.random_bool(f as f64) {
            // specular
            let h = beckmann(rng);
            -v.reflect(h)
        } else if !self.transparent {
            // diffuse
            let dir = random_cosine_weight_on_hemisphere(rng);
            world_onb.transform(dir)
        } else {
            // transmit
            let h = beckmann(rng);
            let cos_v = h.dot(v);
            let v_perp = v - h * cos_v;
            let l_perp = -v_perp / eta_t;
            let sin2_l = l_perp.length_squared();
            if sin2_l > 1.0 {
                return None;
            }
            let cos_l = (1.0 - sin2_l).sqrt();
            -cos_v.signum() * h * cos_l + l_perp
        };

        // Multiple Importance Sampling
        let mut pdf = 0.0;
        pdf += {
            let h = (l + v).normalize();
            let p_h = beckmann_pdf(h);
            // TODO: why abs?
            f * p_h / (4.0 * h.dot(v).abs())
        };
        pdf += if !self.transparent {
            // diffuse component
            (1.0 - f) * n.dot(l).abs() * f32::consts::FRAC_1_PI
        } else if n.dot(v).is_sign_positive() != n.dot(l).is_sign_positive() {
            // transmit component
            let h = (l * eta_t + v).normalize();
            let h_dot_v = h.dot(v);
            let h_dot_l = h.dot(l);
            let jacobian = h_dot_v.abs() / (eta_t * h_dot_l + h_dot_v).powi(2);
            let p_h = beckmann_pdf(h);
            (1.0 - f) * p_h * jacobian
        } else {
            0.0
        };

        Some((l, pdf))
    }
}
