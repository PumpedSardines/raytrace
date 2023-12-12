use crate::{
    object::{
        aabb::AABB,
        hittable::{HitRecord, Hittable},
        material::Material,
    },
    ray::Ray,
};
use vec3::Point3;

#[derive(Clone)]
pub struct Sphere {
    pub material: Material,
    center: Point3<f64>,
    radius: f64,
    bbox: Option<AABB>,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, material: Material) -> Sphere {
        let mut sphere = Sphere {
            center,
            radius,
            material,
            bbox: None,
        };

        sphere.update_bbx();

        sphere
    }

    pub fn set_center(mut self, center: Point3<f64>) -> Sphere {
        self.center = center;
        self.update_bbx();
        self
    }

    pub fn set_radius(mut self, radius: f64) -> Sphere {
        self.radius = radius;
        self.update_bbx();
        self
    }

    fn update_bbx(&mut self) {
        self.bbox = Some(AABB::new(
            self.center - Point3::new(self.radius, self.radius, self.radius),
            self.center + Point3::new(self.radius, self.radius, self.radius),
        ));
    }
}

impl Hittable for Sphere {
    fn bounding_box(&self) -> &Option<AABB> {
        &self.bbox
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = 2.0 * oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant: f64 = b * b - 4.0 * a * c;

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
