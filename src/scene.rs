use crate::{bvh::BvhNode, math::Ray, object::Object, shape::HitRecord};

#[derive(Default)]
pub struct Scene {
    /// The list of objects in the scene
    objects: Vec<Object>,

    /// The BVH for the scene
    bvh: Option<BvhNode>,
}

impl Scene {
    /// Create a empty Object list for Scene
    pub fn new() -> Self {
        Default::default()
    }

    /// Add a Object to Scene
    pub fn add(&mut self, obj: Object) {
        self.objects.push(obj);
        self.bvh = None;
    }

    /// Add a list of Object to Scene
    pub fn add_list<I>(&mut self, obj_list: I)
    where
        I: IntoIterator<Item = Object>,
    {
        self.objects.extend(obj_list);
        self.bvh = None;
    }

    /// Build BVH from current objects which should call after scene setup
    pub fn build_bvh(&mut self) {
        if self.objects.is_empty() {
            self.bvh = None;
            return;
        }
        self.bvh = Some(BvhNode::build(self.objects.clone()));
    }

    /// Get closest intersection of ray with intersectable objects
    pub fn get_closest_intersect(
        &self,
        r: &Ray,
        t_min: f32,
        t_max: f32,
        rec: &mut HitRecord,
    ) -> bool {
        if let Some(bvh) = &self.bvh {
            return bvh.intersect(r, t_min, t_max, rec);
        }
        let mut obj_rec = HitRecord::new();
        let mut hit_any = false;
        let mut closest_so_far = t_max;
        for obj in &self.objects {
            if obj.intersect(r, t_min, closest_so_far, &mut obj_rec) {
                hit_any = true;
                closest_so_far = obj_rec.t;
                *rec = obj_rec.clone();
            }
        }
        hit_any
    }
}
