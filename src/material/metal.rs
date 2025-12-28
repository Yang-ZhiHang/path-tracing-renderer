use crate::{
    material::Material,
    math::{Color, Ray, Vec3, Vec3Ext},
};

#[derive(Clone, Copy)]
pub struct Metal {
    /// The reflectivity to different colors
    pub albedo: Color,

    /// The fuzziness of the reflected rays
    pub fuzz: f32,
}

impl Metal {
    /// Create a metal material from albedo and fuzz
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &crate::shape::HitRecord,
        attenuation: &mut Color,
        scatter: &mut Ray,
    ) -> bool {
        let mut reflect_direction: Vec3 = r_in.direction.reflect(rec.normal);

        // Avoid zero vector
        if reflect_direction.near_zero() {
            reflect_direction = rec.normal;
        }

        *attenuation = self.albedo;
        *scatter = Ray::new(
            rec.p,
            reflect_direction + self.fuzz * Vec3::random_unit_vector(),
            r_in.t,
        );

        // After we add fuzz, we need to ensure the scattered ray is still
        // in outer side of the surface of sphere
        scatter.direction.dot(rec.normal) > 0.0
    }
}
