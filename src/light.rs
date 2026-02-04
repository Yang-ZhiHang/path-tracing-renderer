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
    /// return the color, direction to light and distance to light in micro time.
    pub fn illuminate(&self, pos: Vec3) -> (Color, Vec3, f32) {
        match self {
            Light::Ambient(color) => (*color, Vec3::ZERO, 0.0),
            Light::Directional(color, dir) => (*color, *dir, f32::INFINITY),
            Light::Point(color, loc) => {
                let disp = loc - pos;
                let len = disp.length();
                (*color, disp / len, len)
            }
        }
    }
}
