use std::f32;

use glam::FloatExt;
use rand::{Rng, rngs::StdRng};
use rand_distr::{Distribution, UnitCircle};

use crate::{
    color::{self, Color},
    math::{Vec3, vec3::random_cosine_weight_on_hemisphere},
    onb::ONB,
};

/// Normal Distribution Functions for microfacet distribution.
pub mod ndf {
    use std::f32;

    /// Beckmann distribution.
    /// References:
    /// https://zhuanlan.zhihu.com/p/611622351
    pub fn beckmann(roughness: f32, nh: f32) -> f32 {
        // D = exp(-tanθ_h / m^2) / (π m^2 (n • h)^4)
        // Clamp roughness to avoid division by zero.
        let m2 = roughness * roughness;
        let nh2 = nh * nh;
        let tan2_t = (1.0 - nh2) / nh2;
        (-tan2_t / m2).exp() / (f32::consts::PI * m2 * nh2 * nh2)
    }

    /// GGX (Trowbridge-Reitz) distribution.
    pub fn ggx(roughness: f32, nh: f32) -> f32 {
        let m2 = roughness * roughness;
        let m4 = m2 * m2;
        let nh2 = nh * nh;
        m4 * f32::consts::FRAC_1_PI / ((nh2 * (m4 - 1.0) + 1.0).powi(2))
    }

    /// Blinn-Phong distribution which is used in Unity and UE4.
    /// References:
    /// https://zhuanlan.zhihu.com/p/564814632
    pub fn blinn_phong(roughness: f32, nh: f32) -> f32 {
        let alpha = 2.0 * roughness.powi(2).recip() - 2.0;
        (alpha + 2.0) * f32::consts::FRAC_1_PI * nh.powf(alpha) / 2.0
    }
}

/// Fresnel function for reflectance calculation.
pub mod fresnel {
    use crate::math::Vec3;

    /// Fresnel function using Schlick's approximation.
    /// References:
    /// https://zhuanlan.zhihu.com/p/152226698
    pub fn schlick(index: f32, color: Vec3, metallic: f32, h_dot_v: f32) -> Vec3 {
        // F = F0 + (1 - F0)(1 - v • h)^5
        let f0 = ((index - 1.0) / (index + 1.0)).powi(2);

        let f0 = Vec3::splat(f0).lerp(color, metallic);
        (1.0 - f0).mul_add(Vec3::splat((1.0 - h_dot_v).powi(5)), f0)
    }
}

/// Geometry function for microfacet shadowing.
pub mod gf {
    use crate::math::Vec3;

    /// Smith's method with Schlick-GGX approximation, which is commonly used in path tracing.
    pub fn smith_schlick_ggx(roughness: f32, n: Vec3, l: Vec3, v: Vec3) -> f32 {
        // G = min(1, 2(n • h)(n • wo)/(wo • h), 2(n • h)(n • wi)/(wo • h))
        // let k = (roughness + 1.0).powi(2) / 8.0;
        // k
        let k = roughness.powi(2) / 2.0;
        let g = |v: Vec3| {
            let n_dot_v = n.dot(v).abs();
            n_dot_v / (n_dot_v * (1.0 - k) + k)
        };
        g(l) * g(v)
    }

    /// Cook-Torrance method, which doesn't depend on the shape of NDF, and the calculation is
    /// simple but not as precise physically as Smith's method.
    pub fn cook_torrance(n_dot_h: f32, h_dot_v: f32, n_dot_v: f32, n_dot_l: f32) -> f32 {
        let g = (n_dot_h * n_dot_l).min(n_dot_h * n_dot_v);
        let g = (2.0 * g) / h_dot_v;
        g.min(1.0)
    }
}

pub struct Material {
    /// The base color of the material. Values between (0,0,0) and (1,1,1).
    pub color: Color,

    /// The roughness of the material. Values will be automatically clamped between 0.01 and 1.0.
    pub roughness: f32,

    /// The metallic property of the material. Values between 0.0 and 1.0.
    pub metallic: f32,

    /// The index of refraction of the material.
    pub index: f32,

    /// The emittance of the material. Used for light sources.
    pub emittance: f32,

    /// Whether the material is transparent.
    pub transparent: bool,
}

impl Material {
    /// Base constructor with default values, sanitized roughness and index of refraction.
    pub fn base(index: f32, roughness: f32) -> Self {
        Self {
            color: color::WHITE,
            // Clamp roughness to avoid numerical issues in NDF.
            // roughness should be at least 0.01 to avoid glass sphere to be too dark.
            roughness: roughness.clamp(1e-2, 1.0),
            metallic: 0.0,
            // Avoid index of exactly 1.0 to prevent numerical issues in refraction calculations.
            index: index + 1e-6,
            emittance: 0.0,
            transparent: false,
        }
    }

    /// Specular reflection material with specified color.
    pub fn specular(color: Color, roughness: f32) -> Self {
        Self {
            color,
            ..Self::base(1.0, roughness)
        }
    }

    /// Specular diffuse material with specified color.
    pub fn diffuse(color: Color) -> Self {
        Self {
            color,
            ..Self::base(1.0, 1.0)
        }
    }

    /// Transparent glass material with specified roughness and index of refraction.
    pub fn clear(index: f32, roughness: f32) -> Self {
        Self {
            transparent: true,
            ..Self::base(index, roughness)
        }
    }

    /// Metal material with specified color and roughness.
    pub fn metallic(color: Color, roughness: f32) -> Self {
        Self {
            color,
            metallic: 1.0,
            ..Self::base(1.0, roughness)
        }
    }

    /// Light material with specified color and emittance.
    pub fn light(color: Color, emittance: f32) -> Self {
        Self {
            color,
            emittance,
            ..Self::base(1.0, 1.0)
        }
    }

    /// Colored transparent material
    pub fn transparent(color: Color, index: f32, roughness: f32) -> Self {
        Self {
            color,
            transparent: true,
            ..Self::base(index, roughness)
        }
    }
}

impl Material {
    /// Bi-direction Scatter Distribution Function. Including BRDF and BTDF.
    ///
    /// Parameters:
    /// - l: The direction from the intersection point towards light.
    /// - v: The direction from the intersection point towards view.
    /// - n: The normal vector of the surface of the intersection point.
    /// - front_face: Whether the incident ray is towards the outside of the surface. We use
    ///   front_face instead of checking the sign of n.dot(v) because we have already flipped
    ///   the normal
    ///
    /// Returning the function describes the distribution of scattering.
    pub fn bsdf(&self, l: Vec3, v: Vec3, n: Vec3, front_face: bool) -> Vec3 {
        // normal distribution function
        let ndf = |nh| ndf::beckmann(self.roughness, nh);
        let gf = |n, l, v| gf::smith_schlick_ggx(self.roughness, n, l, v);

        let n_dot_l = n.dot(l);
        let n_dot_v = n.dot(v);
        let l_outside = n_dot_l.is_sign_positive();
        let v_outside = n_dot_v.is_sign_positive();

        if l_outside == v_outside {
            // Reflection

            // For reflection, h is the half vector between l and v
            let h = (l + v).normalize();
            let n_dot_h = n.dot(h);
            let h_dot_v = v.dot(h);

            // 1. normal distribution function
            let d = ndf(n_dot_h);
            // 2. fresnel function
            let f = if !l_outside && (1.0 - n_dot_v * n_dot_v).sqrt() * self.index > 1.0 {
                Vec3::splat(1.0)
            } else {
                fresnel::schlick(self.index, self.color, self.metallic, h_dot_v)
            };
            // 3. geometry function
            let g = gf(n, l, v);

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
                self.index
            } else {
                1.0 / self.index
            };

            // For refraction, h is -(η_i * l + η_o * v).normalize()
            let h = -(l * eta_t + v).normalize();

            let n_dot_h = n.dot(h);
            let h_dot_v = v.dot(h);
            let h_dot_l = l.dot(h);

            // 1. normal distribution function
            let d = ndf(n_dot_h);
            // 2. fresnel function
            let f = fresnel::schlick(self.index, self.color, self.metallic, h_dot_v.abs());
            // 3. geometry function
            let g = gf(n, l, v);

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
        // front_face equals to -v.dot(n).is_sign_negative() which implemented in `shape.rs`.
        let eta_t = if front_face {
            self.index
        } else {
            1.0 / self.index
        };

        // Estimate specular contribution using Fresnel.
        let f0 = ((self.index - 1.0) / (self.index + 1.0)).powi(2);
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
                // Total internal reflection.
                // We should not return reflection for `l` here, as this branch is for transmission only.
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
            f * p_h / (4.0 * h.dot(v).abs())
        };
        pdf += if !self.transparent {
            // diffuse component
            (1.0 - f) * n.dot(l).abs() * f32::consts::FRAC_1_PI
        } else if n.dot(v).is_sign_positive() != n.dot(l).is_sign_positive() {
            // transmit component
            let h = -(l * eta_t + v).normalize();
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
