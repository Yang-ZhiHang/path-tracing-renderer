use std::f32::consts::PI;
use std::path::Path;

use image::{DynamicImage, ImageReader};
use palette::{LinSrgb, Srgb};

use crate::math::{Color, Vec3};
use crate::texture::Texture;

pub struct ImageTexture {
    image: RtwImage,
}

impl ImageTexture {
    pub fn load<P>(path: P) -> image::ImageResult<Self>
    where
        P: AsRef<Path>,
    {
        Ok(Self {
            image: RtwImage::from_path(path)?,
        })
    }
}

impl Texture for ImageTexture {
    /// Get the sphere coordinates u, v which ranged in [0, 1) from intersection point.
    /// Theta is the angle from pole -Y axis to +Y axis. And phi is the angle from -X axis to
    /// +Z axis and then to -X axis.
    fn get_color(&self, p: Vec3) -> Color {
        // Normalize to make UV mapping independent of radius length
        let p = p.normalize();
        let phi = (-p.z).atan2(p.x) + PI;
        let theta = (-p.y).acos();
        let u = phi / (2.0 * PI);
        let v = theta / PI;
        self.image.sample(u, v)
    }
}

struct RtwImage {
    width: u32,
    height: u32,
    data: Vec<Color>,
}

impl RtwImage {
    fn from_path<P: AsRef<Path>>(path: P) -> image::ImageResult<Self> {
        let dyn_img = ImageReader::open(path)?.decode()?;
        Self::from_dynamic(dyn_img)
    }

    fn from_dynamic(img: DynamicImage) -> image::ImageResult<Self> {
        match img {
            DynamicImage::ImageRgb32F(rgb) => Ok(Self::from_linear_rgb(rgb)),
            DynamicImage::ImageRgba32F(rgba) => Ok(Self::from_linear_rgba(rgba)),
            _ => {
                let rgb = img.to_rgb8();
                let (width, height) = rgb.dimensions();
                let mut data = Vec::with_capacity((width * height) as usize);

                for p in rgb.pixels() {
                    let srgb: Srgb<u8> = Srgb::new(p[0], p[1], p[2]);
                    let LinSrgb {
                        red, green, blue, ..
                    } = srgb.into_format::<f32>().into_linear();
                    data.push(Color::new(red, green, blue));
                }

                Ok(Self {
                    width,
                    height,
                    data,
                })
            }
        }
    }

    fn from_linear_rgb(rgb: image::Rgb32FImage) -> Self {
        let (width, height) = rgb.dimensions();
        let mut data = Vec::with_capacity((width * height) as usize);

        for p in rgb.pixels() {
            data.push(Color::new(p[0], p[1], p[2]));
        }

        Self {
            width,
            height,
            data,
        }
    }

    fn from_linear_rgba(rgba: image::Rgba32FImage) -> Self {
        let (width, height) = rgba.dimensions();
        let mut data = Vec::with_capacity((width * height) as usize);

        for p in rgba.pixels() {
            data.push(Color::new(p[0], p[1], p[2])); // drop alpha
        }

        Self {
            width,
            height,
            data,
        }
    }

    fn sample(&self, u: f32, v: f32) -> Color {
        if self.data.is_empty() || self.width == 0 || self.height == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = (u * (self.width - 1) as f32) as u32;
        let j = (v * (self.height - 1) as f32) as u32;
        let idx = (j * self.width + i) as usize;

        self.data
            .get(idx)
            .copied()
            .unwrap_or_else(|| Color::new(0.0, 1.0, 1.0))
    }
}
