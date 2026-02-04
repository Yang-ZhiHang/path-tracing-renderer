use std::f32;

use crate::{color::Color, math::Vec3};

pub enum Light {
    // Ambient light with color
    Ambient(Color),

    // Directional light with color and direction vector
    Directional(Color, Vec3),

    // Point light with color and location in world coordinate.
    Point(Color, Vec3),
}

impl Light {
    /// Illuminates a point, returning the intensity, direction to the light and distance to the light in micro time.
    pub fn illuminate(&self, pos: Vec3) -> (Vec3, Vec3, f32) {
        match self {
            Light::Ambient(color) => (*color, Vec3::ZERO, 0.0),
            Light::Directional(color, dir) => (*color, *dir, f32::INFINITY),
            Light::Point(color, loc) => {
                let disp = loc - pos;
                let len = disp.length();
                // The point light source attenuates 1/r^2 for displacement r.
                (*color / (len * len), disp / len, len)
            }
        }
    }
}
