use glam::f32::*;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub(crate) struct Camera {
    pub origin: Vec3A,
    pub viewport_upper_left: Vec3A,
    pub pixel_delta_u: Vec3A,
    pub pixel_delta_v: Vec3A,
    pub image_width: u32,
    pub image_height: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Uniforms {
    pub seed: u32,
    pub sphere_count: u32,
    pub samples: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub(crate) struct Material {
    pub albedo: Vec3A,
    pub roughness: f32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub(crate) struct Sphere {
    pub center: Vec3A,
    pub radius: f32,
    pub material: Material,
}

pub(crate) trait ToTypeMapping {
    type TypeMapping;

    fn to_type_mapping(&self) -> Self::TypeMapping;
}
