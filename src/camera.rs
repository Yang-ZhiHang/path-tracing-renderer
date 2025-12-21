use crate::math::{Point3, Ray, Vec3};

pub struct Camera {
    /// The original point of camera
    pub origin: Point3,

    /// The length of focal which refers to the distance from origin
    /// in the reverse direction of axis z
    pub focal_length: f32,

    /// Width of viewport
    pub horizontal: Vec3,

    /// Height of viewport
    pub vertical: Vec3,

    /// The bottom left corner of viewport which centered on the origin
    pub lower_left: Point3,
}

impl Camera {
    pub fn new(
        origin: [f32; 3],
        focal_length: f32,
        viewport_height: f32,
        viewport_width: f32,
    ) -> Self {
        let origin = Point3::from_array(origin);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        Self {
            origin,
            focal_length,
            horizontal,
            vertical,
            lower_left: origin
                - horizontal / 2.0
                - vertical / 2.0
                - Point3::new(0.0, 0.0, focal_length),
        }
    }

    /// Returns a ray start from origin according to u, v which ranged in [0, 1].
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
