use crate::math::{Point3, Ray, Vec3};

pub mod sphere;

#[derive(Default, Clone)]
pub struct HitRecord {
    /// The origin coordinate of light ray
    pub p: Point3,

    /// Time
    pub t: f32,

    /// Normal vector
    pub normal: Vec3,

    /// If the normal vector towards you
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Default::default()
    }

    /// Let normal vector face to the coming ray
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hitable>>,
}

pub type Scene = HittableList;

impl HittableList {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add<T>(&mut self, obj: T)
    where
        T: Hitable + 'static,
    {
        self.objects.push(Box::new(obj));
    }

    pub fn add_list<T, I>(&mut self, obj_list: I)
    where
        T: Hitable + 'static,
        I: IntoIterator<Item = T>,
    {
        for obj in obj_list {
            self.objects.push(Box::new(obj));
        }
    }
}

impl Hitable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut obj_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for obj in &self.objects {
            if obj.hit(r, t_min, closest_so_far, &mut obj_rec) {
                hit_anything = true;
                closest_so_far = obj_rec.t;
                *rec = obj_rec.clone();
            }
        }
        hit_anything
    }
}
