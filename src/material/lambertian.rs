use std::f32;
use std::sync::Arc;

use crate::color::{self, Color};
use crate::material::Material;
use crate::math::vec3::random_cosine_weight_on_hemisphere;
use crate::math::{Ray, Vec3Ext};
use crate::onb::ONB;
use crate::shape::HitRecord;
use crate::texture::Texture;
use crate::texture::solid_color::SolidColor;

#[derive(Clone)]
pub struct Lambertian {
    /// The texture of the material.
    pub tex: Arc<dyn Texture>,
}

impl Default for Lambertian {
    /// Create a default lambertian material with gray albedo.
    fn default() -> Self {
        Self {
            tex: Arc::new(SolidColor::new(color::WHITE)),
        }
    }
}

impl Lambertian {
    /// Create a lambertian material from albedo.
    pub fn new(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }

    /// Create a lambertian material from texture.
    pub fn from_texture<T>(tex: T) -> Self
    where
        T: Texture + 'static,
    {
        Self { tex: Arc::new(tex) }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let uvw = ONB::new(rec.normal);
        let mut scatter_direction = uvw.transform(random_cosine_weight_on_hemisphere());

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let attenuation = self.tex.sample(rec.u, rec.v, rec.normal);
        let scatter = Ray::new(rec.p, scatter_direction, r_in.t);
        Some((attenuation, scatter))
    }

    fn brdf(&self, _r_in: &Ray, r_out: &Ray, rec: &HitRecord) -> f32 {
        let cos = rec.normal.dot(r_out.dir.normalize());
        cos / f32::consts::PI
    }
}
