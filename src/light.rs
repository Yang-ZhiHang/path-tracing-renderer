use std::f64;

use glam::DVec3;
use rand::rngs::StdRng;

use crate::{color::Color, object::Object};

pub enum Light {
    /// Ambient light with color
    Ambient(Color),

    /// Directional light with color and direction vector
    Directional(Color, DVec3),

    /// Point light with color and location in world coordinate.
    Point(Color, DVec3),

    /// Light from invisible, emissive object
    Object(Object),
}

impl Light {
    /// Illuminates a point.
    /// Returning the intensity, direction from `pos` to the light and distance from `pos` to the light in micro time.
    pub fn illuminate(
        &self,
        pos: DVec3,
        rng: &mut StdRng,
        shutter_time: f64,
    ) -> (DVec3, DVec3, f64) {
        match self {
            Light::Ambient(color) => (*color, DVec3::ZERO, 0.0),
            Light::Directional(color, dir) => (
                *color,
                // The dir means the direction from the light to the point.
                // So we need to negate it to get the direction from the point to the light.
                -*dir,
                f64::INFINITY,
            ),
            Light::Point(color, loc) => {
                let disp = loc - pos;
                let len = disp.length();
                (
                    // The point light source attenuates 1/r^2 for displacement r.
                    *color / (len * len),
                    disp / len,
                    len,
                )
            }
            Light::Object(object) => {
                let (p, n, pdf) = object.shape.sample(pos, rng, shutter_time);
                let disp = p - pos;
                let len = disp.length();
                // Only consider the light if it's facing the point.
                let cosine = (-disp.dot(n)).max(0.0) / len;
                let surface_area = cosine / (len * len);
                (
                    object.material.emittance * object.material.color * surface_area / pdf,
                    disp / len,
                    len,
                )
            }
        }
    }
}
