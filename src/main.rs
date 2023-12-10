mod color;
mod hittable;
mod image;
mod material;
mod random;
mod ray;
mod sphere;
mod vec3;

use crate::image::Image;
use color::Color;
use hittable::HitRecord;
use indicatif::ProgressBar;
use material::Material;
use random::Random;
use ray::Ray;
use sphere::Sphere;
use vec3::{Point3, Vec3};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1080;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let focal_length = 2.0;
    let viewport_height = 2.0;
    let samples_per_pixel = 10;
    let bounce_limit = 10;

    let viewport_width = (image_width as f64 / image_height as f64) * viewport_height;
    let camera_center = Point3::new(0.0, 0.0, 1.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u.scalar(1.0 / image_width as f64);
    let pixel_delta_v = viewport_v.scalar(1.0 / image_height as f64);

    let viewport_upper_left = camera_center
        - viewport_u.scalar(0.5)
        - viewport_v.scalar(0.5)
        - Vec3::new(0.0, 0.0, focal_length);

    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v).scalar(0.5);

    let mut image = Image::new(image_width, image_height);
    let mut rng = Random::new(256);

    let progress_bar = ProgressBar::new((image_height * image_width) as u64 * samples_per_pixel);

    for y in 0..image_height {
        for x in 0..image_width {
            let mut pixel_color = Color::black();

            for _ in 0..samples_per_pixel {
                progress_bar.inc(1);
                let px = -0.5 + rng.next();
                let py = -0.5 + rng.next();

                let pixel_center = pixel00_loc
                    + (pixel_delta_u.scalar(x as f64 + px))
                    + (pixel_delta_v.scalar(y as f64 + py));
                let ray_direction = pixel_center - camera_center;

                let r = Ray::new(camera_center, ray_direction);
                pixel_color = pixel_color + ray_color(r, bounce_limit, &mut rng);
            }

            image.set_pixel(
                x,
                y,
                (pixel_color * Color::grey(1.0 / samples_per_pixel as f64)).linear_to_gamma(),
            );
        }
    }

    image.save_png("image.png");
}

fn ray_color(ray: Ray, depth: u8, rng: &mut Random) -> Color {
    if depth == 0 {
        return Color::black();
    }

    let objects: Vec<Box<dyn hittable::Hittable>> = vec![
        Box::new(Sphere::new(
            Point3::new(-1.86, 0.1, -2.0),
            0.6,
            Material::new()
                .set_albedo(Color::new(1.0, 0.0, 1.0))
                .set_roughness(0.6),
        )),
        Box::new(Sphere::new(
            Point3::new(-0.93, -0.2, -2.0),
            0.3,
            Material::new()
                .set_albedo(Color::white())
                .set_roughness(0.3),
        )),
        Box::new(Sphere::new(
            Point3::new(-0.23, -0.25, -0.5),
            0.15,
            Material::new()
                .set_albedo(Color::white())
                .set_roughness(0.1),
        )),
        Box::new(Sphere::new(
            Point3::new(0.23, -0.25, -0.5),
            0.15,
            Material::new()
                .set_albedo(Color::white())
                .set_roughness(0.1),
        )),
        Box::new(Sphere::new(
            Point3::new(-0.31, -0.1, -1.5),
            0.4,
            Material::new().set_albedo(Color::new(0.1, 1.0, 0.1)),
        )),
        Box::new(Sphere::new(
            Point3::new(0.31, -0.2, -2.0),
            0.3,
            Material::new()
                .set_albedo(Color::new(0.1, 0.1, 1.0))
                .set_roughness(0.6),
        )),
        Box::new(Sphere::new(
            Point3::new(0.63, -0.2, -1.0),
            0.3,
            Material::new()
                .set_albedo(Color::new(1.0, 1.0, 0.2))
                .set_roughness(1.0),
        )),
        Box::new(Sphere::new(
            Point3::new(1.56, -0.2, -2.0),
            0.3,
            Material::new().set_albedo(Color::new(1.0, 0.2, 0.2)),
        )),
        Box::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Material::new().set_albedo(Color::new(0.2, 0.5, 1.0)),
        )),
        Box::new(Sphere::new(
            Point3::new(0.0, 1060.5, -1.0),
            100.0,
            Material::new().set_emission(Color::new(1.0, 0.7, 0.2)),
        )),
    ];

    let mut current_hit_info: Option<HitRecord> = None;
    for object in objects {
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
    if let Some(hit_info) = current_hit_info {
        if let Some((color, ray)) = hit_info.material.scatter(&ray, &hit_info, rng) {
            let light_strength = hit_info.normal.dot(ray.direction).abs();

            let background =
                color * ray_color(ray, depth - 1, rng) * Color::grey(light_strength * 0.5);

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
