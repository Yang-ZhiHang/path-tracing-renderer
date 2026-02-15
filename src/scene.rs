use crate::color::Color;
use crate::light::Light;
use crate::{bvh::BvhNode, object::Object};

#[derive(Default)]
pub struct Scene {
    /// The list of objects in the scene.
    pub objects: Vec<Object>,

    /// The list of lights in the scene.
    pub lights: Vec<Light>,

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
    pub fn with_obj(mut self, obj: Object) -> Self {
        self.objects.push(obj);
        self
    }

    /// Builder-style batch add that consumes and returns the Scene.
    pub fn with_obj_list<I>(mut self, obj_list: I) -> Self
    where
        I: IntoIterator<Item = Object>,
    {
        self.objects.extend(obj_list);
        self
    }

    /// Add a Light to Scene.
    pub fn with_light(mut self, light: Light) -> Self {
        self.lights.push(light);
        self
    }

    /// Add a list of Light to Scene.
    pub fn with_lights<I>(mut self, lights: I) -> Self
    where
        I: IntoIterator<Item = Light>,
    {
        self.lights.extend(lights);
        self
    }

    /// Build BVH from current objects which should call after scene setup.
    /// After built the BVH, you can't add more objects or lights to scene. Or else you should call this function again.
    pub fn build_bvh(mut self) -> Self {
        if self.objects.is_empty() {
            self.bvh = None;
        } else {
            self.bvh = Some(BvhNode::build(self.objects.clone()));
        }
        self
    }
}
