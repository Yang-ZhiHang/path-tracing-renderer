use std::f32;

use rand::rngs::StdRng;

use crate::{color::Color, math::Vec3, object::Object};

pub enum Light {
    /// Ambient light with color
    Ambient(Color),

    /// Directional light with color and direction vector
    Directional(Color, Vec3),

    /// Point light with color and location in world coordinate.
    Point(Color, Vec3),

    /// Light from invisible, emissive object
    Object(Object),
}

impl Light {
    /// Illuminates a point.
    /// Returning the intensity, direction from `pos` to the light and distance from `pos` to the light in micro time.
    pub fn illuminate(&self, pos: Vec3, rng: &mut StdRng) -> (Vec3, Vec3, f32) {
        match self {
            Light::Ambient(color) => (*color, Vec3::ZERO, 0.0),
            Light::Directional(color, dir) => (*color, *dir, f32::INFINITY),
            Light::Point(color, loc) => {
                let disp = loc - pos;
                let len = disp.length();
                // The point light source attenuates 1/r^2 for displacement r.
                (*color / (len * len), disp / len, len)
            }
            Light::Object(object) => {
                let (p, n, pdf) = object.shape.sample(pos, rng);
                let disp = p - pos;
                let len = disp.length();
                let cosine = (-disp.dot(n)).max(0.0) / len;
                let surface_area = cosine.max(0.0) / (len * len);
                (
                    object.material.emittance * object.material.color * surface_area / pdf,
                    disp / len,
                    len,
                )
            }
        }
    }
}
