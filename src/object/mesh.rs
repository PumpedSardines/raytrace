use crate::{
    object::{
        hittable::{BVHNode, HitRecord, Hittable, HittableList},
        material::Material,
        triangle::Triangle,
    },
    ray::Ray,
};

#[derive(Clone)]
pub struct Mesh {
    pub material: Material,
    pub triangles: BVHNode,
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle>, material: Material) -> Mesh {
        let mut list = HittableList::new(vec![]);
        for triangle in triangles {
            list.add(Box::new(triangle));
        }

        Mesh {
            material,
            triangles: BVHNode::new(&list),
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
        let hit_info = self.triangles.hit(&r, t_min, t_max);

        hit_info.map(|hit_info| HitRecord {
            t: hit_info.t,
            p: hit_info.p,
            normal: hit_info.normal,
            front_face: hit_info.front_face,
            material: self.material,
        })
    }
}
