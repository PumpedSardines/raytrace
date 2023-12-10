mod camera;
mod color;
mod hittable;
mod image;
mod material;
mod random;
mod ray;
mod scene;
mod sphere;
mod vec3;

use camera::{Camera, RenderOptions};
use color::Color;
use hittable::HitRecord;
use image::Image;
use indicatif::ProgressBar;
use material::Material;
use random::Random;
use ray::Ray;
use scene::Scene;
use sphere::Sphere;
use vec3::{Point3, Vec3};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1000;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 2;

    let camera = Camera {
        origin: Point3::new(2.0, 2.0, 2.0),
        direction: Vec3::new(-1.0, -1.0, -1.0),
        focal_length: 1.0,
        image_width: image_width,
        image_height: image_height,
        viewport_width: 2.0,
    };

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

    let mut image = Image::new(image_width as u32, image_height as u32);

    let progress_bar =
        ProgressBar::new((image_height * image_width) as u64 * samples_per_pixel as u64);

    progress_bar.inc(1);

    camera.render(
        &Scene::new(objects),
        &RenderOptions {
            samples_per_pixel: samples_per_pixel,
            bounce_limit: 50,
        },
        |(x, y), c| {
            image.set_pixel(x, y, c);
            progress_bar.inc(1);
        },
    );

    image.save_png("image.png");
}
