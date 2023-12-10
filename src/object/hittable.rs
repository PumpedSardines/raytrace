use vec3::{Point3, Vec3};

use crate::object::material::Material;
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub t: f64,
    pub p: Point3<f64>,
    pub normal: Vec3<f64>,
    pub front_face: bool,
    pub material: Material,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
