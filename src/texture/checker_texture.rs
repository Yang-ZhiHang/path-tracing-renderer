use std::sync::Arc;

use crate::{
    math::{Color, Point3},
    texture::{Texture, solid_color::SolidColor},
};

#[derive(Clone)]
pub struct CheckerTexture {
    pub inv_scale: f32,
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f32, c1: Color, c2: Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            odd: Arc::new(SolidColor::new(c1)),
            even: Arc::new(SolidColor::new(c2)),
        }
    }

    pub fn from_textures<T>(scale: f32, odd: T, even: T) -> Self
    where
        T: Texture + 'static,
    {
        Self {
            inv_scale: 1.0 / scale,
            odd: Arc::new(odd),
            even: Arc::new(even),
        }
    }
}

impl Texture for CheckerTexture {
    fn sample(&self, u: f32, v: f32, p: Point3) -> Color {
        // let ix = (self.inv_scale * p.x).floor() as i32;
        // let iy = (self.inv_scale * p.y).floor() as i32;
        // let iz = (self.inv_scale * p.z).floor() as i32;

        // if (ix + iy + iz) & 1 == 0 {
        //     self.even.sample(u, v, p)
        // } else {
        //     self.odd.sample(u, v, p)
        // }
        let iu = (self.inv_scale * u).floor() as i32;
        let iv = (self.inv_scale * v).floor() as i32;
        if (iu + iv) & 1 == 0 {
            self.even.sample(u, v, p)
        } else {
            self.odd.sample(u, v, p)
        }
    }
}
