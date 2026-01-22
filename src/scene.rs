use crate::color::Color;
use crate::{bvh::BvhNode, object::Object};

#[derive(Default)]
pub struct Scene {
    /// The list of objects in the scene.
    pub objects: Vec<Object>,

    /// The list of lights in the scene.
    pub lights: Vec<Object>,

    /// The BVH for the scene.
    pub bvh: Option<BvhNode>,

    /// The background color of the scene
    pub background: Color,
}

impl Scene {
    /// Create a empty scene.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the background of the scene.
    pub const fn background(mut self, color: Color) -> Self {
        self.background = color;
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

    /// Add a `Object` to Scene.
    pub fn add(&mut self, obj: Object) -> &mut Self {
        self.objects.push(obj);
        self.bvh = None;
        self
    }

    /// Add a list of `Object` to Scene.
    pub fn add_list<I>(&mut self, obj_list: I) -> &mut Self
    where
        I: IntoIterator<Item = Object>,
    {
        self.objects.extend(obj_list);
        self.bvh = None;
        self
    }

    /// Add a Light to Scene.
    pub fn add_light(&mut self, light: Object) -> &mut Self {
        self.lights.push(light);
        self.bvh = None;
        self
    }

    /// Add a list of Light to Scene.
    pub fn add_lights<I>(&mut self, lights: I) -> &mut Self
    where
        I: IntoIterator<Item = Object>,
    {
        self.lights.extend(lights);
        self.bvh = None;
        self
    }

    /// Consume the builder and return the Scene.
    pub const fn build(self) -> Self {
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
