use std::sync::Arc;

use crate::{
    aabb::Aabb,
    material::{Material, lambertian::Lambertian},
    shape::{Bounded, HitRecord, Hittable},
};

#[derive(Clone)]
pub struct Object {
    /// The shape of object
    pub shape: Arc<dyn Bounded>,

    /// The material of object
    pub material: Arc<dyn Material>,
}

impl Object {
    /// Create a Object from shape with default lambertian material
    pub fn new<T>(shape: T) -> Object
    where
        T: Bounded + 'static,
    {
        Self {
            shape: Arc::new(shape),
            material: Arc::new(Lambertian::default()),
        }
    }

    /// Set material for object
    pub fn material<T>(mut self, material: T) -> Object
    where
        T: Material + 'static,
    {
        self.material = Arc::new(material);
        self
    }
}

impl Hittable for Object {
    /// Get HitRecord of ray with object
    fn intersect(&self, r: &crate::math::Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        rec.material = Some(self.material.clone());
        self.shape.intersect(r, t_min, t_max, rec)
    }
}

impl Bounded for Object {
    /// Get axis-aligned bounding box of object
    fn bbox(&self) -> Aabb {
        self.shape.bbox()
    }
}
