use crate::{
    camera::Camera,
    color::Color,
    gpu::type_mapping,
    gpu::type_mapping::ToTypeMapping,
    objects::{Obj, ToObj},
};

pub struct World {
    pub(crate) spheres: Vec<type_mapping::Sphere>,
    pub(crate) camera: type_mapping::Camera,
}

impl World {
    pub fn new() -> Self {
        Self {
            spheres: Vec::new(),
            camera: Camera::new().to_type_mapping(),
        }
    }

    pub fn with_camera(mut self, camera: Camera) -> Self {
        self.camera = camera.to_type_mapping();
        self
    }

    pub fn with_object(mut self, object: impl ToObj) -> Self {
        self.add_obj(object.to_obj());
        self
    }

    pub fn with_objects(mut self, objects: Vec<Box<dyn ToObj>>) -> Self {
        for object in objects {
            self.add_obj(object.to_obj());
        }
        self
    }

    pub fn add_object(&mut self, object: impl ToObj) {
        self.add_obj(object.to_obj());
    }

    fn add_obj(&mut self, obj: Obj) {
        match obj {
            Obj::Sphere(sphere) => self.spheres.push(sphere.to_type_mapping()),
        }
    }

    pub fn render(&self) -> Vec<Color> {
        crate::gpu::ray_trace::render(&self)
    }
}
