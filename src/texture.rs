pub mod checker_texture;
pub mod image_texture;
pub mod solid_color;

use crate::math::{Color, Vec3};

pub trait Texture: Send + Sync {
    fn sample(&self, u: f32, v: f32, p: Vec3) -> Color;
}
