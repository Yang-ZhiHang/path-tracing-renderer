use std::sync::Arc;

use crate::{
    aabb::Aabb,
    color,
    interval::Interval,
    material::Material,
    math::Ray,
    shape::{Bounded, HitRecord, Hittable},
};

#[derive(Clone)]
pub struct Object {
    /// The shape of object
    pub shape: Arc<dyn Bounded>,

    /// The material of object
    pub material: Arc<Material>,
}

impl Object {
    /// Create a Object from shape with default lambertian material
    pub fn new<T>(shape: T) -> Self
    where
        T: Bounded + 'static,
    {
        Self {
            shape: Arc::new(shape),
            material: Arc::new(Material::diffuse(color::GREY)),
        }
    }

    /// Set material for object
    pub fn material(mut self, material: Material) -> Self {
        self.material = Arc::new(material);
        self
    }
}

impl Hittable for Object {
    /// Set the material for `rec` and call `intersect` of the member `shape`.
    fn intersect(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        rec.material = Some(self.material.clone());
        self.shape.intersect(r, ray_t, rec)
    }
}

impl Bounded for Object {
    /// Get axis-aligned bounding box of object
    fn bbox(&self) -> Aabb {
        self.shape.bbox()
    }
}
