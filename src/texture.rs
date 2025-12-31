pub mod checker_texture;
pub mod image_texture;
pub mod solid_color;

use crate::math::{Color, Vec3};

pub trait Texture: Send + Sync {
    fn get_color(&self, p: Vec3) -> Color;
}

// TODO: No solid color texture, just make trait image texture struct
// TODO: buffer store image
