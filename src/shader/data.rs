use crate::camera;
use glam::f32::*;

pub struct RenderData {
    pub camera: camera::Camera,
    pub spheres: Vec<Sphere>,
}

#[repr(C)]
pub struct Uniforms {
    pub seed: u32,
    pub sphere_count: u32,
    pub samples: u32,
}

#[repr(C)]
pub struct Sphere {
    pub origin: Vec3A,
    pub radius: f32,
}

/// This is the camera data that is passed to the shader.
/// Should never be constructed manually
#[repr(C)]
pub(crate) struct CameraBake {
    pub(crate) origin: Vec3A,
    pub(crate) viewport_upper_left: Vec3A,
    pub(crate) pixel_delta_u: Vec3A,
    pub(crate) pixel_delta_v: Vec3A,
    pub(crate) image_width: u32,
    pub(crate) image_height: u32,
}
