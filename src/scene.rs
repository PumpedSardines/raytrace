use crate::{
    color::Gradient,
    object::hittable::{HitRecord, Hittable},
};

pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
    pub sky_color: Gradient,
}

impl Hittable for Scene {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;

        // First loop through all objects to find the closest hit.
        for object in &self.objects {
            if let Some(hit_info) = object.hit(&ray, t_min, t_max) {
                if let Some(c_hit_info) = closest_hit {
                    if hit_info.t < c_hit_info.t {
                        closest_hit = Some(hit_info);
                    }
                } else {
                    closest_hit = Some(hit_info);
                }
            }
        }

        closest_hit
    }
}
