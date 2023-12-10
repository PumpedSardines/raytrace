use crate::{
    color::Color,
    hittable::HitRecord,
    random::Random,
    ray::Ray,
    scene::Scene,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    pub origin: Point3<f64>,
    pub direction: Vec3<f64>,
    pub focal_length: f64,
    pub image_width: u32,
    pub image_height: u32,
    pub viewport_width: f64,
}

pub struct RenderOptions {
    pub samples_per_pixel: usize,
    pub bounce_limit: u8,
}

impl Camera {
    pub fn viewport_upper_left(&self) -> Point3<f64> {
        self.origin - self.viewport_u().scalar(0.5) - self.viewport_v().scalar(0.5)
            + self.direction.normalized().scalar(self.focal_length)
    }

    pub fn viewport_u(&self) -> Vec3<f64> {
        let x = 1.0;
        let y = 0.0;
        let z = -1.0 * (x * self.direction.x) / self.direction.z;

        Vec3::new(x, y, z).normalized().scalar(self.viewport_width)
    }

    pub fn viewport_v(&self) -> Vec3<f64> {
        let viewport_height =
            (self.viewport_width / self.image_width as f64) * self.image_height as f64;
        self.viewport_u()
            .cross(self.direction)
            .normalized()
            .scalar(-1.0 * viewport_height)
    }

    pub fn pixel_delta_u(&self) -> Vec3<f64> {
        self.viewport_u().scalar(1.0 / self.image_width as f64)
    }

    pub fn pixel_delta_v(&self) -> Vec3<f64> {
        self.viewport_v().scalar(1.0 / self.image_height as f64)
    }

    pub fn pixel_location(&self, x: f64, y: f64) -> Point3<f64> {
        self.viewport_upper_left()
            + self.pixel_delta_u().scalar(x)
            + self.pixel_delta_v().scalar(y)
            + (self.pixel_delta_u() + self.pixel_delta_v()).scalar(0.5)
    }

    pub fn render(
        &self,
        scene: &Scene,
        options: &RenderOptions,
        mut cb: impl FnMut((u32, u32), Color) -> (),
    ) {
        if self.direction == Vec3::new(0.0, 0.0, 0.0) {
            panic!("Camera direction cannot be zero vector.");
        }

        let mut rng = Random::new(256);
        let samples_per_pixel_f64 = options.samples_per_pixel as f64;

        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let mut pixel_color = Color::black();

                for _ in 0..options.samples_per_pixel {
                    // Antialiasing
                    let px = -0.5 + rng.next();
                    let py = -0.5 + rng.next();

                    let pixel_center = self.pixel_location(x as f64 + px, y as f64 + py);
                    let ray_direction = pixel_center - self.origin;

                    let r = Ray::new(self.origin, ray_direction);
                    pixel_color =
                        pixel_color + Self::ray_color(scene, r, options.bounce_limit, &mut rng);
                }

                let color =
                    (pixel_color * Color::grey(1.0 / samples_per_pixel_f64)).linear_to_gamma();
                cb((x, y), color);
            }
        }
    }

    /// Calculate the color of a ray.
    fn ray_color(scene: &Scene, ray: Ray, depth: u8, rng: &mut Random) -> Color {
        if depth == 0 {
            return Color::black();
        }

        let mut current_hit_info: Option<HitRecord> = None;

        // First loop thorugh all objects to find the closest hit.
        for object in &scene.objects {
            if let Some(hit_info) = object.hit(&ray, 0.001, f64::INFINITY) {
                if let Some(c_hit_info) = current_hit_info {
                    if hit_info.t < c_hit_info.t {
                        current_hit_info = Some(hit_info);
                    }
                } else {
                    current_hit_info = Some(hit_info);
                }
            }
        }

        // If we have a hit, calculate the color.
        if let Some(hit_info) = current_hit_info {
            if let Some((color, ray)) = hit_info.material.scatter(&ray, &hit_info, rng) {
                let light_strength = hit_info.normal.dot(ray.direction).abs();

                let background = color
                    * Self::ray_color(scene, ray, depth - 1, rng)
                    * Color::grey(light_strength * 0.5);

                if let Some(emission) = hit_info.material.emission {
                    return emission + (Color::white() - emission) * background;
                }

                return background;
            } else {
                return Color::black();
            }
        }

        let unit_direction = ray.direction.normalized();
        let t = 0.5 * (unit_direction.y + 1.0);

        let white = Color::white();
        let blue = Color::new(0.5, 0.7, 1.0);

        white * (Color::grey(1.0 - t)) + blue * (Color::grey(t))
    }
}
