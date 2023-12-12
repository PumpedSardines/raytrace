use crate::{
    object::{
        hittable::{HitRecord, Hittable},
        material::Material,
        triangle::Triangle,
    },
    ray::Ray,
};

pub struct Mesh {
    pub material: Material,
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle>, material: Material) -> Mesh {
        Mesh {
            material,
            triangles,
        }
    }

    pub fn from_obj(obj: &obj::Obj, material: Material) -> Mesh {
        let mut triangles = Vec::new();

        obj.face_iter().for_each(|face| {
            triangles.push(Triangle::new(
                [face[0].into(), face[1].into(), face[2].into()],
                Material::new(),
            ));
        });

        Mesh::new(triangles, material)
    }
}

impl Hittable for Mesh {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record = None;

        for triangle in &self.triangles {
            if let Some(record) = triangle.hit(r, t_min, closest_so_far) {
                closest_so_far = record.t;
                hit_record = Some(record);
            }
        }

        hit_record.map(|mut record| {
            record.material = self.material;
            record
        });

        hit_record
    }
}
