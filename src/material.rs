use super::{color::Color, hittable::HitRecord, random::Random, ray::Ray, vec3::Vec3};

#[derive(Clone, Copy)]
pub struct Material {
    pub albedo: Color,
    pub roughness: f64,
    pub emission: Color,
}

impl Material {
    pub fn new(albedo: Color, roughness: f64, emission) -> Material {
        Material { albedo, roughness, emission }
    }

    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut Random) -> Option<(Color, Ray)> {
        let random_direction = rec.normal + rng.unit_vec3();
        let reflected_direction = r_in.direction.normalized().reflect(rec.normal);
        let scatter_direction = Vec3::lerp(
            reflected_direction,
            random_direction,
            self.roughness.powi(2),
        );

        let scattered = Ray::new(rec.p, scatter_direction);

        Some((self.albedo, scattered))
    }
}
