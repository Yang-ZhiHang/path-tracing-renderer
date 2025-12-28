use crate::{
    common::random,
    material::Material,
    math::{Color, Ray, Vec3},
    shape::HitRecord,
};

#[derive(Clone, Copy)]
pub struct Dielectric {
    pub indices_of_refraction: f32,
}

impl Dielectric {
    pub fn new(indices_of_refraction: f32) -> Self {
        Self {
            indices_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scatter: &mut Ray,
    ) -> bool {
        let ir = if rec.front_face {
            1.0 / self.indices_of_refraction
        } else {
            self.indices_of_refraction
        };
        let unit_direction = r_in.direction.normalize();

        // Ensure the incident ray has refract ray
        let cos_theta = -unit_direction.dot(rec.normal);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let refractable = ir * sin_theta < 1.0;

        // The smaller the cosine, the larger the incident angle, and
        // the more reflection component the human eye sees
        let direction: Vec3 = if refractable
            && Self::reflectance(&self, cos_theta, self.indices_of_refraction) <= random()
        {
            unit_direction.refract(rec.normal, ir)
        } else {
            unit_direction.reflect(rec.normal)
        };

        *attenuation = Color::splat(1.0);
        *scatter = Ray::new(rec.p, direction, rec.t);
        true
    }
}
