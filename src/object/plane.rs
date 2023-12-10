use vec3::Vec3;

use crate::{
    object::{
        hittable::{HitRecord, Hittable},
        material::Material,
    },
    ray::Ray,
};

pub struct Plane {
    pub distance_from_origin: f64,
    pub normal: Vec3<f64>,
    pub material: Material,
}

impl Plane {
    pub fn new(distance_from_origin: f64, normal: Vec3<f64>, material: Material) -> Plane {
        Plane {
            distance_from_origin,
            normal,
            material,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let a = r.direction.dot(self.normal);

        if a.abs() < 0.0001 {
            return None;
        }

        let b = self.distance_from_origin - r.origin.dot(self.normal);

        let t = b / a;

        if t <= 0.0 {
            return None;
        } else {
            if t < t_min || t > t_max {
                return None;
            }

            let front_face = r.direction.dot(self.normal) < 0.0;

            Some(HitRecord {
                t,
                p: r.at(t),
                normal: if front_face {
                    self.normal.normalized()
                } else {
                    -self.normal.normalized()
                },
                front_face,
                material: self.material,
            })
        }
    }
}
