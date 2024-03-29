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
    pub plane_count: u32,
    pub triangle_count: u32,
    pub samples: u32,
    pub max_bounces: u32,
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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub(crate) struct Plane {
    pub normal: Vec3A,
    pub distance: f32,
    pub material: Material,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub(crate) struct Triangle {
    pub point_a: Vec3A,
    pub point_b: Vec3A,
    pub point_c: Vec3A,
    pub normal: Vec3A,
    pub distance: f32,
    pub material: Material,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            albedo: Vec3A::new(0.0, 0.0, 0.0),
            roughness: 0.0,
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: Vec3A::new(0.0, 0.0, 0.0),
            radius: 0.0,
            material: Material::default(),
        }
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            normal: Vec3A::new(0.0, 0.0, 0.0),
            distance: 0.0,
            material: Material::default(),
        }
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            point_a: Vec3A::new(0.0, 0.0, 0.0),
            point_b: Vec3A::new(0.0, 0.0, 0.0),
            point_c: Vec3A::new(0.0, 0.0, 0.0),
            normal: Vec3A::new(0.0, 0.0, 0.0),
            distance: 0.0,
            material: Material::default(),
        }
    }
}

pub(crate) trait ToTypeMapping {
    type TypeMapping;

    fn to_type_mapping(&self) -> Self::TypeMapping;
}
