use image::RgbImage;
use indicatif::ProgressBar;
use rayon::prelude::*;

use crate::buffer::Buffer;
use crate::camera::Camera;
use crate::color::{self, Color};
use crate::interval::Interval;
use crate::math::Ray;
use crate::math::random;
use crate::scene::Scene;
use crate::shape::{HitRecord, Hittable};

pub struct Renderer {
    /// The camera to use
    pub cam: Camera,

    /// The scene to render
    pub scene: Scene,

    /// The width of output image
    pub width: u32,

    /// The height of output image
    pub height: u32,

    /// The number of samplings for one pixel in an image.
    pub num_samples: u32,

    /// The maximum number of the light bounces in the image.
    pub max_bounces: u32,

    /// The progress bar to show
    pub pb: Option<ProgressBar>,
}

impl Renderer {
    /// Create a renderer from camera and scene.
    pub const fn new(cam: Camera, scene: Scene) -> Self {
        Self {
            cam,
            scene,
            width: 800,
            height: 600,
            pb: None,
            max_bounces: 50,
            num_samples: 100,
        }
    }

    /// Set the width of output image.
    pub const fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    /// Set the height of output image.
    pub const fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    /// Set progress bar for renderer.
    pub fn progress_bar(mut self, pb: ProgressBar) -> Self {
        self.pb = Some(pb);
        self
    }

    /// Set number of samplings for one pixel.
    pub const fn num_samples(mut self, n: u32) -> Self {
        self.num_samples = n;
        self
    }

    /// Set maximum number of the light bounces for renderer.
    pub const fn max_bounces(mut self, n: u32) -> Self {
        self.max_bounces = n;
        self
    }

    /// Trace the ray and return the color.
    pub fn trace_ray(&self, ray: &Ray, num_bounces: u32, rec: &mut HitRecord) -> Color {
        if num_bounces == 0 {
            return color::BLACK;
        }

        // Start ray interval above zero to avoid shadow acne.
        if !self.intersect(ray, Interval::new(1e-3, f32::INFINITY), rec) {
            return self.scene.background;
        }

        let color_from_emission = rec.material.as_ref().unwrap().emit(rec.u, rec.v, rec.p);

        // The material could use `unwrap` instead of `map_or` because it will not be `None` if
        // scene.intersect is true.
        match rec.material.as_ref().unwrap().scatter(ray, rec) {
            Some((attenuation, mut r_out)) => {
                let light = &self.scene.lights[0];
                r_out = Ray::new(rec.p, light.shape.random(rec.p), ray.t);
                let pdf = light.shape.pdf(&r_out);
                let f_r = rec.material.as_ref().unwrap().brdf(ray, &r_out, rec);
                let color_from_scatter =
                    attenuation * f_r * self.trace_ray(&r_out, num_bounces - 1, rec) / pdf;

                color_from_emission + color_from_scatter
            }
            None => color_from_emission,
        }
    }

    // /// Sample the ray towards lights and return the scattered ray and pdf.
    // fn sample_lights(&self, rec: &HitRecord, shutter_time: f32) -> (Ray, Color) {
    //     for light in &self.scene.lights {
    //         let r_out = light.shape.random(rec);
    //         let pdf = light.shape.pdf(&Ray::new(rec.p, r_out, shutter_time));
    //     }
    //     todo!()
    // }

    /// Get the pixel color of a specified location in film plane.
    pub fn get_color(&self, col: u32, row: u32, iterations: u32) -> Color {
        let mut pixel_color = Color::default();
        let mut rec = HitRecord::default();
        // Sampling stratifications + Mento Carlo approximatiom.
        let iter_sqrt = (iterations as f32).sqrt() as u32;
        for y in 0..iter_sqrt {
            for x in 0..iter_sqrt {
                let s = (col as f32 + (x as f32 + random()) / iter_sqrt as f32) / self.width as f32;
                let t =
                    (row as f32 + (y as f32 + random()) / iter_sqrt as f32) / self.height as f32;
                let r = self.cam.get_ray(s, t);
                pixel_color += self.trace_ray(&r, self.max_bounces, &mut rec);
            }
        }
        pixel_color / iterations as f32
    }

    /// Get all pixel colors in film plane and store into `buffer`.
    pub fn sample(&self, iterations: u32, buffer: &mut Buffer) {
        let colors: Vec<_> = (0..self.height)
            .into_par_iter()
            .map(|row| {
                let row_pixels: Vec<Color> = (0..self.width)
                    .map(|col| self.get_color(col, row, iterations))
                    .collect();

                // Update progress bar after finish each row
                if let Some(pb) = self.pb.as_ref() {
                    pb.inc(1);
                }
                row_pixels
            })
            .collect();
        buffer.extend(colors);
        if let Some(pb) = self.pb.as_ref() {
            pb.finish_with_message("Done!");
        }
    }

    /// Render the image for given scene and return `RgbImage`.
    pub fn render(&self) -> RgbImage {
        let mut buffer = Buffer::new(self.width, self.height);
        self.sample(self.num_samples, &mut buffer);
        buffer.image()
    }
}

impl Hittable for Renderer {
    /// Get closest intersection of ray with intersectable objects.
    fn intersect(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if let Some(bvh) = &self.scene.bvh {
            return bvh.intersect(r, ray_t, rec);
        }
        let mut obj_rec = HitRecord::default();
        let mut hit_any = false;
        let mut closest_so_far = ray_t.max;
        for obj in &self.scene.objects {
            let search_interval = Interval::new(ray_t.min, closest_so_far);
            if obj.intersect(r, search_interval, &mut obj_rec) {
                hit_any = true;
                closest_so_far = obj_rec.t;
                *rec = obj_rec.clone();
            }
        }
        hit_any
    }
}
