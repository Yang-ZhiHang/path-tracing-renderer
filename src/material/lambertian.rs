use std::sync::Arc;

use crate::material::Material;
use crate::math::{Color, Ray, Vec3, Vec3Ext};
use crate::shape::HitRecord;
use crate::texture::Texture;
use crate::texture::solid_color::SolidColor;

#[derive(Clone)]
pub struct Lambertian {
    /// The texture representing the albedo of the material
    pub tex: Arc<dyn Texture>,
}

impl Lambertian {
    /// Create a lambertian material from albedo
    pub fn new(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }

    /// Create a default lambertian material with gray albedo
    pub fn default() -> Self {
        Self {
            tex: Arc::new(SolidColor::new(Color::splat(0.5))),
        }
    }

    /// Create a lambertian material from texture
    pub fn from_texture<T>(tex: T) -> Self
    where
        T: Texture + 'static,
    {
        Self { tex: Arc::new(tex) }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scatter: &mut Ray,
    ) -> bool {
        *attenuation = self.tex.get_color(rec.normal);
        *scatter = Ray::new(rec.p, rec.normal + Vec3::random_unit_vector(), r_in.t);
        true
    }
}
