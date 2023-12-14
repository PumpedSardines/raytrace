use crate::{color::Color, gpu::ToTypeMapping};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Material {
    /// The color of the material
    pub albedo: Color,
    /// The roughness of the material
    pub roughness: f32,
}

impl Material {
    pub fn new() -> Self {
        Self {
            albedo: Color::white(),
            roughness: 1.0,
        }
    }

    pub fn with_albedo(mut self, albedo: Color) -> Self {
        self.albedo = albedo;
        self
    }

    pub fn with_roughness(mut self, roughness: f32) -> Self {
        self.roughness = roughness;
        self
    }
}

impl ToTypeMapping for Material {
    type TypeMapping = crate::gpu::type_mapping::Material;

    fn to_type_mapping(&self) -> Self::TypeMapping {
        Self::TypeMapping {
            albedo: self.albedo.to_vec3a(),
            roughness: self.roughness,
        }
    }
}
