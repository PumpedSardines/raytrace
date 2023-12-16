use crate::{
    camera::Camera,
    gpu::type_mapping,
    gpu::type_mapping::ToTypeMapping,
    objects::{Obj, ToObj},
};

pub struct World {
    pub(crate) spheres: Vec<type_mapping::Sphere>,
    pub(crate) planes: Vec<type_mapping::Plane>,
    pub(crate) triangles: Vec<type_mapping::Triangle>,
    pub(crate) camera: type_mapping::Camera,
}

impl World {
    pub fn new() -> Self {
        Self {
            spheres: Vec::new(),
            planes: Vec::new(),
            triangles: Vec::new(),
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
            Obj::Plane(plane) => self.planes.push(plane.to_type_mapping()),
            Obj::Triangle(triangle) => self.triangles.push(triangle.to_type_mapping()),
        }
    }
}
