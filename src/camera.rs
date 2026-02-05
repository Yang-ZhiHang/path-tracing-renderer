use crate::{
    math::random,
    math::{Point3, Ray, Vec3, vec3::random_in_unit_disk},
};

#[allow(non_snake_case)]
pub struct Camera {
    /// The original point of camera.
    pub origin: Point3,

    /// Camera coordinate system basis vectors in x axis.
    pub c_x: Vec3,

    /// Camera coordinate system basis vectors in y axis (from bottom to top).
    pub c_y: Vec3,

    /// The width of viewport.
    pub viewport_width: f32,

    /// The height of viewport.
    pub viewport_height: f32,

    /// Pixel coordinate system basis vectors in x axis.
    pub u: Vec3,

    /// Pixel coordinate system basis vectors in y axis (from top to bottom).
    pub v: Vec3,

    /// The bottom left corner of viewport which centered on the origin.
    pub upper_left: Point3,

    /// Lens radius for depth of field effect.
    pub lens_radius: f32,
}

impl Camera {
    #[allow(non_snake_case)]
    pub fn new(
        look_from: Point3,
        look_to: Point3,
        vup: Vec3, // View up vector
        vfov: f32, // Vertical field-of-view in degrees
        aspect_ratio: f32,
        aperture: f32,
        focal_length: f32,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * aspect_ratio;

        // Camera coordinates
        let c_z: Vec3 = (look_from - look_to).normalize();
        let c_x: Vec3 = vup.cross(c_z).normalize();
        let c_y: Vec3 = c_z.cross(c_x);

        // Pixel coordinates
        let u: Vec3 = viewport_width * c_x;
        let v: Vec3 = viewport_height * -c_y;
        let upper_left: Point3 = look_from - u / 2.0 - v / 2.0 - c_z * focal_length;

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
    pub fn get_ray(&self, i: f32, j: f32) -> Ray {
        let mut lens_offset = self.lens_radius * random_in_unit_disk();
        lens_offset = self.c_x * lens_offset.x + self.c_y * lens_offset.y;
        let dir = self.upper_left + i * self.u + j * self.v - self.origin - lens_offset;
        let shutter_time = random();
        Ray::new(self.origin + lens_offset, dir.normalize(), shutter_time)
    }

    /// Get how much width for one pixel.
    pub fn pixel_delta_u(&self, image_width: u32) -> Vec3 {
        self.viewport_width * self.c_x / image_width as f32
    }

    /// Get how much height for one pixel.
    pub fn pixel_delta_v(&self, image_height: u32) -> Vec3 {
        self.viewport_height * self.c_y / image_height as f32
    }
}
