use super::hittable::{HitRecord, Hittable};
use super::material::Material;
use super::ray::Ray;
use super::vec3::{Point3, Vec3};

pub struct Sphere {
    pub center: Point3<f64>,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = 2.0 * oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant <= 0.0 {
            return None;
        } else {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);

            if t < t_min || t > t_max {
                return None;
            }

            let outward_normal = (r.at(t) - self.center).scalar(1.0 / self.radius);
            let front_face = r.direction.dot(outward_normal) < 0.0;

            Some(HitRecord {
                t,
                p: r.at(t),
                normal: if front_face {
                    outward_normal.normalized()
                } else {
                    -outward_normal.normalized()
                },
                front_face,
                material: self.material,
            })
        }
    }
}
