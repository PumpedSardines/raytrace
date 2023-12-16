use crate::{gpu::ToTypeMapping, material::Material, position::Position};

use super::{Obj, ToObj};

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub points: [Position; 3],
    pub material: Material,
}

impl ToObj for Triangle {
    fn to_obj(&self) -> Obj {
        Obj::Triangle(self.clone())
    }
}

impl ToTypeMapping for Triangle {
    type TypeMapping = crate::gpu::type_mapping::Triangle;

    fn to_type_mapping(&self) -> Self::TypeMapping {
        let v = [
            self.points[0].to_vec3a(),
            self.points[1].to_vec3a(),
            self.points[2].to_vec3a(),
        ];

        let v1 = v[1] - v[0];
        let v2 = v[2] - v[0];

        let normal = v1.cross(v2).normalize();
        let distance = -normal.dot(v[0]);

        Self::TypeMapping {
            point_a: v[0],
            point_b: v[1],
            point_c: v[2],
            normal,
            distance,
            material: self.material.to_type_mapping(),
        }
    }
}
