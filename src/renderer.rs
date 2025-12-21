use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use indicatif::ProgressBar;

use crate::camera::Camera;
use crate::common::random;
use crate::math::{Color, ColorExt, Ray};
use crate::shape::{HitRecord, Hitable, Scene};

pub struct Renderer {
    /// The camera to use
    pub cm: Camera,

    /// The scene to render
    pub scene: Scene,

    /// The progress bar to show
    pub pb: Option<ProgressBar>,

    /// The path to save image
    pub output_path: PathBuf,

    /// To eliminate jaggies
    pub samples_per_pixel: u32,
}

impl Renderer {
    pub fn new(
        cm: Camera,
        scene: Scene,
        pb: Option<ProgressBar>,
        output_path: PathBuf,
        samples_per_pixel: u32,
    ) -> Self {
        Self {
            cm,
            scene,
            pb,
            output_path,
            samples_per_pixel,
        }
    }

    pub fn sample(r: &Ray, obj: &Scene) -> Color {
        let mut rec = HitRecord::new();
        if obj.hit(r, 0.0, f32::INFINITY, &mut rec) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }
        let unit_direction = r.direction.normalize();
        let t: f32 = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::rgb(1.0, 1.0, 1.0) + t * Color::rgb(0.2, 0.5, 1.0)
    }

    pub fn write_color(io: &mut impl std::io::Write, color: &Color, samples_per_pixel: u32) {
        let mut ir = color.r();
        let mut ig = color.g();
        let mut ib = color.b();

        let scale = 1.0 / samples_per_pixel as f32;
        ir *= scale;
        ig *= scale;
        ib *= scale;

        // Translate [0, 1) values to [0, 255]
        io.write_all(
            format!(
                "{} {} {}\n",
                (256.0 * ir.clamp(0.0, 0.999)) as u8,
                (256.0 * ig.clamp(0.0, 0.999)) as u8,
                (256.0 * ib.clamp(0.0, 0.999)) as u8
            )
            .as_bytes(),
        )
        .expect("Failed to write color");
    }

    pub fn render(&self, image_width: u32, image_height: u32) {
        let mut file = File::create(&self.output_path).expect("Create image file failed.");

        // Header of image which format in PPM
        writeln!(file, "P3\n{} {}\n255", image_width, image_height)
            .expect("Failed to write PPM header");

        for row in (0..image_height).rev() {
            for col in 0..image_width {
                let mut pixel_color = Color::default();
                for _ in 0..self.samples_per_pixel {
                    let u = (col as f32 + random()) / (image_width - 1) as f32;
                    let v = (row as f32 + random()) / (image_height - 1) as f32;
                    let r = self.cm.get_ray(u, v);
                    pixel_color += Self::sample(&r, &self.scene);
                }
                Renderer::write_color(&mut file, &pixel_color, self.samples_per_pixel);
            }
            self.pb.as_ref().map(|pb| pb.inc(1));
        }
        self.pb.as_ref().map(|pb| pb.finish_with_message("Done!"));
    }
}
