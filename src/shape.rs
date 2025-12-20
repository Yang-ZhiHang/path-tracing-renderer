use crate::math::{Point3, Ray, Vec3};

pub mod sphere;

#[derive(Default, Clone)]
/// Why we need a record struct for ray hit?
/// When we scan the viewport from camera, we should compare each object and know
/// which t is the nearest so that we can shade the proper color.
///
/// A HitRecord contains p and t.
/// A point in the ray can be performed like: p = origin + t * direction
/// coord: p, origin
/// vector: direction
/// variable: t
pub struct HitRecord {
    pub p: Point3,
    pub t: f32,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new() -> Self {
        Default::default()
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

#[derive(Default)]
pub struct Scene {
    objects: Vec<Box<dyn Hitable>>,
}

impl Scene {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add<T: Hitable + 'static>(&mut self, obj: T) {
        self.objects.push(Box::new(obj));
    }
}

impl Hitable for Scene {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut obj_rec = HitRecord::new();
        let mut hit_anything = false;
        for obj in &self.objects {
            if obj.hit(r, t_min, t_max, &mut obj_rec) {
                hit_anything = true;
                if obj_rec.t < rec.t {
                    *rec = obj_rec.clone();
                }
            }
        }
        hit_anything
    }
}
