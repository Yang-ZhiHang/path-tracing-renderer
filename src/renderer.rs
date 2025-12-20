use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use indicatif::ProgressBar;

use crate::camera::Camera;
use crate::math::{Color, ColorExt, Ray};
use crate::shape::{HitRecord, Hitable, Scene};

pub struct Renderer {
    /// The camera to use
    pub cm: Camera,

    /// The scene to render
    pub scene: Scene,

    /// The progress bar to show
    pub pb: ProgressBar,

    /// The path to save image
    pub output_path: PathBuf,
}

impl Renderer {
    pub fn new(cm: Camera, scene: Scene, pb: ProgressBar, output_path: PathBuf) -> Self {
        Self {
            cm,
            scene,
            pb,
            output_path,
        }
    }

    pub fn sample(r: &Ray, obj: &Scene) -> Color {
        let mut rec = HitRecord::new();
        if obj.hit(r, 0.0, std::f32::INFINITY, &mut rec) {
            return Color::rgb(255.0, 0.0, 0.0);
        }
        let unit_direction = r.direction().normalize();
        let t: f32 = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::rgb(1.0, 1.0, 1.0) + t * Color::rgb(0.2, 0.5, 1.0)
    }

    pub fn write_color(io: &mut impl std::io::Write, color: &Color) {
        let ir = (color.r().clamp(0.0, 0.999) * 255.99) as u8;
        let ig = (color.g().clamp(0.0, 0.999) * 255.99) as u8;
        let ib = (color.b().clamp(0.0, 0.999) * 255.99) as u8;
        io.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())
            .expect("Failed to write color");
    }

    pub fn render(&self, image_width: u32, image_height: u32) {
        let mut file = File::create(&self.output_path).expect("Create image file failed.");

        // Header of image which format in PPM
        writeln!(file, "P3\n{} {}\n255", image_width, image_height)
            .expect("Failed to write PPM header");

        for row in (0..image_height).rev() {
            for col in 0..image_width {
                let u = col as f32 / (image_width - 1) as f32;
                let v = row as f32 / (image_height - 1) as f32;
                let r = Ray::new(
                    self.cm.origin,
                    self.cm.lower_left + u * self.cm.horizontal + v * self.cm.vertical
                        - self.cm.origin,
                );
                let pixel_color = Self::sample(&r, &self.scene);
                Renderer::write_color(&mut file, &pixel_color);
            }
            self.pb.inc(1);
        }
        self.pb.finish_with_message("Done!");
    }
}
