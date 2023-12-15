use crate::{gpu::ToTypeMapping, material::Material, position::Vector};

use super::{Obj, ToObj};

#[derive(Clone, Copy, Debug)]
pub struct Plane {
    pub normal: Vector,
    pub distance: f32,
    pub material: Material,
}

impl ToObj for Plane {
    fn to_obj(&self) -> Obj {
        Obj::Plane(self.clone())
    }
}

impl ToTypeMapping for Plane {
    type TypeMapping = crate::gpu::type_mapping::Plane;

    fn to_type_mapping(&self) -> Self::TypeMapping {
        Self::TypeMapping {
            normal: self.normal.to_vec3a(),
            distance: self.distance,
            material: self.material.to_type_mapping(),
        }
    }
}
