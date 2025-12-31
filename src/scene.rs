use crate::interval::Interval;
use crate::shape::Hittable;
use crate::{bvh::BvhNode, math::Ray, object::Object, shape::HitRecord};

#[derive(Default)]
pub struct Scene {
    /// The list of objects in the scene.
    objects: Vec<Object>,

    /// The BVH for the scene.
    bvh: Option<BvhNode>,
}

impl Scene {
    /// Create a empty Object list for Scene.
    pub fn new() -> Self {
        Default::default()
    }

    /// Add a Object to Scene.
    pub fn add(&mut self, obj: Object) -> &mut Self {
        self.objects.push(obj);
        self.bvh = None;
        self
    }

    /// Add a list of Object to Scene.
    pub fn add_list<I>(&mut self, obj_list: I) -> &mut Self
    where
        I: IntoIterator<Item = Object>,
    {
        self.objects.extend(obj_list);
        self.bvh = None;
        self
    }

    /// Builder-style add that consumes and returns the Scene.
    pub fn with(mut self, obj: Object) -> Self {
        self.objects.push(obj);
        self.bvh = None;
        self
    }

    /// Builder-style add_list that consumes and returns the Scene.
    pub fn with_list<I>(mut self, obj_list: I) -> Self
    where
        I: IntoIterator<Item = Object>,
    {
        self.objects.extend(obj_list);
        self.bvh = None;
        self
    }

    /// Consume the builder and return the Scene.
    pub fn build(self) -> Self {
        self
    }

    /// Build BVH from current objects which should call after scene setup.
    pub fn build_bvh(&mut self) {
        if self.objects.is_empty() {
            self.bvh = None;
            return;
        }
        self.bvh = Some(BvhNode::build(self.objects.clone()));
    }
}

impl Hittable for Scene {
    /// Get closest intersection of ray with intersectable objects.
    fn intersect(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if let Some(bvh) = &self.bvh {
            return bvh.intersect(r, ray_t, rec);
        }
        let mut obj_rec = HitRecord::new();
        let mut hit_any = false;
        let mut closest_so_far = ray_t.max;
        for obj in &self.objects {
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
