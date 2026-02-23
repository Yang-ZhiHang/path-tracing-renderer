use glam::DVec3;
use rand::rngs::StdRng;
use rand_distr::{Distribution, UnitDisc};

use crate::math::{DPoint3, Ray, random};

#[allow(non_snake_case)]
pub struct Camera {
    /// The original point of camera.
    pub origin: DPoint3,

    /// Camera coordinate system basis vectors in x axis.
    pub c_x: DVec3,

    /// Camera coordinate system basis vectors in y axis (from bottom to top).
    pub c_y: DVec3,

    /// The width of viewport.
    pub viewport_width: f64,

    /// The height of viewport.
    pub viewport_height: f64,

    /// Pixel coordinate system basis vectors in x axis.
    pub u: DVec3,

    /// Pixel coordinate system basis vectors in y axis (from top to bottom).
    pub v: DVec3,

    /// The bottom left corner of viewport which centered on the origin.
    pub upper_left: DPoint3,

    /// Lens radius for depth of field effect.
    pub lens_radius: f64,
}

impl Camera {
    #[allow(non_snake_case)]
    pub fn new(
        look_from: DPoint3,
        look_to: DPoint3,
        vup: DVec3, // View up vector
        vfov: f64,  // Vertical field-of-view in degrees
        aspect_ratio: f64,
        aperture: f64,
        focal_length: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * aspect_ratio;

        // Camera coordinates
        let c_z: DVec3 = (look_from - look_to).normalize();
        let c_x: DVec3 = vup.cross(c_z).normalize();
        let c_y: DVec3 = c_z.cross(c_x);

        // Pixel coordinates
        let u: DVec3 = viewport_width * c_x;
        let v: DVec3 = viewport_height * -c_y;
        let upper_left: DPoint3 = look_from - u / 2.0 - v / 2.0 - c_z * focal_length;

        let lens_radius = aperture / 2.0;

        Self {
            origin: look_from,
            c_x,
            c_y,
            viewport_width,
            viewport_height,
            u,
            v,
            upper_left,
            lens_radius,
        }
    }

    /// Get the ray from aperture to pixel plane.
    /// The pixel plane uses coordinate (i, j) which ranged between [0, 1).
    pub fn get_ray(&self, i: f64, j: f64, rng: &mut StdRng) -> Ray {
        let [x, y]: [f64; 2] = UnitDisc.sample(rng);
        let mut lens_offset = self.lens_radius * DVec3::new(x, y, 0.0);
        lens_offset = self.c_x * lens_offset.x + self.c_y * lens_offset.y;
        let dir = self.upper_left + i * self.u + j * self.v - self.origin - lens_offset;
        let shutter_time = random();
        Ray::new(self.origin + lens_offset, dir.normalize(), shutter_time)
    }

    /// Get how much width for one pixel.
    pub fn pixel_delta_u(&self, image_width: u32) -> DVec3 {
        self.viewport_width * self.c_x / image_width as f64
    }

    /// Get how much height for one pixel.
    pub fn pixel_delta_v(&self, image_height: u32) -> DVec3 {
        self.viewport_height * self.c_y / image_height as f64
    }
}
