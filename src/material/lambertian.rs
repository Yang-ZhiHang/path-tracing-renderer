use crate::material::Material;
use crate::math::{Color, Ray, Vec3, Vec3Ext};

#[derive(Clone, Copy)]
pub struct Lambertian {
    /// The reflectivity to different colors
    pub albedo: Color,
}

impl Lambertian {
    /// Create a lambertian material from albedo
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    /// Create a default lambertian material with gray albedo
    pub fn default() -> Self {
        Self {
            albedo: Color::splat(0.5),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &crate::shape::HitRecord,
        attenuation: &mut Color,
        scatter: &mut Ray,
    ) -> bool {
        *attenuation = self.albedo;
        *scatter = Ray::new(rec.p, rec.normal + Vec3::random_unit_vector(), r_in.t);
        true
    }
}
