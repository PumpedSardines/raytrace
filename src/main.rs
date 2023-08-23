use std::fs;

mod color;
mod hittable;
mod image;
mod random;
mod ray;
mod sphere;
mod vec3;

use color::Color;
use image::Image;
use random::Random;
use ray::Ray;
use sphere::Sphere;
use vec3::{Point3, Vec3};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1080;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = (image_width as f64 / image_height as f64) * viewport_height;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

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

    for y in 0..image_height {
        for x in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u.scalar(x as f64)) + (pixel_delta_v.scalar(y as f64));
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(r);

            image.set_pixel(x, y, pixel_color);
        }
    }

    fs::write("image.ppm", image.to_ppm()).expect("Could not write image file");
}

fn ray_color(ray: Ray) -> Color {
    let objects: Vec<Box<dyn hittable::Hittable>> = vec![
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    for object in objects {
        if let Some(hit_info) = object.hit(&ray, 0.0, f64::INFINITY) {
            let r = (hit_info.normal.x + 1.0) * 0.5;
            let g = (hit_info.normal.y + 1.0) * 0.5;
            let b = (hit_info.normal.z + 1.0) * 0.5;
            return Color::new((255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8);
        }
    }

    let unit_direction = ray.direction.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);

    let white = Color::white();
    let blue = Color::new(128, 180, 255);

    white.mul(1.0 - t) + blue.mul(t)
}
