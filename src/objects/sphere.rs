use crate::{gpu::ToTypeMapping, material::Material, position::Position};

use super::{Obj, ToObj};

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub origin: Position,
    pub radius: f32,
    pub material: Material,
}

impl ToObj for Sphere {
    fn to_obj(&self) -> Obj {
        Obj::Sphere(self.clone())
    }
}

impl ToTypeMapping for Sphere {
    type TypeMapping = crate::gpu::type_mapping::Sphere;

    fn to_type_mapping(&self) -> Self::TypeMapping {
        Self::TypeMapping {
            center: self.origin.to_vec3a(),
            radius: self.radius,
            material: self.material.to_type_mapping(),
        }
    }
}
