use crate::{
    math::{Color, Point3},
    texture::Texture,
};

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColor {
    fn get_color(&self,  _p: Point3) -> Color {
        self.albedo
    }
}
