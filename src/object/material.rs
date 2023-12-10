use crate::{color::Color, object::hittable::HitRecord, ray::Ray};

use random::Random;
use vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Material {
    pub albedo: Color,
    pub roughness: f64,
    pub emission: Option<Color>,
}

impl Material {
    pub fn new() -> Material {
        Material {
            albedo: Color::new(0.7, 0.7, 0.7),
            roughness: 1.0,
            emission: None,
        }
    }

    pub fn set_albedo(mut self, albedo: Color) -> Material {
        self.albedo = albedo;
        self
    }

    pub fn set_emission(mut self, color: Color) -> Material {
        self.emission = Some(color);
        self
    }

    pub fn set_roughness(mut self, roughness: f64) -> Material {
        self.roughness = roughness;
        self
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
