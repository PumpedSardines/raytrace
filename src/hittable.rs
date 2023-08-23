use super::ray::Ray;
use super::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub t: f64,
    pub p: Point3<f64>,
    pub normal: Vec3<f64>,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
