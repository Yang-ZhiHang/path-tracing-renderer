use crate::math::Vec3;

///  Ortho-Normal Basis including u, v, w represents the components of the x, y, z axes
pub struct ONB {
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl ONB {
    /// Create a new ortho-normal basis from given axis vector.
    pub fn new(n: Vec3) -> Self {
        let w = n.normalize();
        let r = if w.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = w.cross(r).normalize();
        let u = w.cross(v);
        Self { u, v, w }
    }

    /// Transform the coordinates of vec to ortho-normal basis coordinates.
    pub fn transform(&self, vec: Vec3) -> Vec3 {
        vec.x * self.u + vec.y * self.v + vec.z * self.w
    }
}
