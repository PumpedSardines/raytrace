use crate::{gpu::ToTypeMapping, material::Material, position::Position};

use super::{Obj, ToObj, Triangle};

#[derive(Clone)]
pub struct Mesh {
    triangles: Vec<crate::gpu::type_mapping::Triangle>,
    material: Material,
}

impl Mesh {
    pub fn new(material: Material) -> Self {
        Self {
            triangles: vec![],
            material,
        }
    }

    pub fn add_triangle(&mut self, points: [Position; 3]) {
        let triangle = Triangle {
            points,
            material: self.material.clone(),
        }
        .to_type_mapping();

        self.triangles.push(triangle);
    }

    pub fn from_obj(obj: &str, material: Material) -> Self {
        let mut mesh = Mesh::new(material);
        let mut vertexes = vec![];
        let mut faces = vec![];

        for line in obj.lines() {
            if line.starts_with("v ") {
                let mut coords = line.split_whitespace().skip(1);
                let x = coords.next().unwrap().parse::<f32>().unwrap();
                let y = coords.next().unwrap().parse::<f32>().unwrap();
                let z = coords.next().unwrap().parse::<f32>().unwrap();

                vertexes.push((x, y, z));
            }

            if line.starts_with("f ") {
                let mut coords = line.split_whitespace().skip(1);
                let point0 = coords.next().unwrap().parse::<usize>().unwrap();
                let point1 = coords.next().unwrap().parse::<usize>().unwrap();
                let point2 = coords.next().unwrap().parse::<usize>().unwrap();

                faces.push([point0, point1, point2]);
            }
        }

        for face in faces {
            let point0 = vertexes[face[0] - 1];
            let point1 = vertexes[face[1] - 1];
            let point2 = vertexes[face[2] - 1];

            let points = [
                Position::new(point0.0, point0.1, point0.2),
                Position::new(point1.0, point1.1, point1.2),
                Position::new(point2.0, point2.1, point2.2),
            ];

            mesh.add_triangle(points);
        }

        mesh
    }
}

impl ToObj for Mesh {
    fn to_obj(&self) -> Obj {
        Obj::Mesh(self.clone())
    }
}

impl ToTypeMapping for Mesh {
    type TypeMapping = Vec<crate::gpu::type_mapping::Triangle>;

    fn to_type_mapping(&self) -> Self::TypeMapping {
        self.triangles.clone()
    }
}
