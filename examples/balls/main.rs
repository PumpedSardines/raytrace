use image::Image;
use ray_trace::camera::{Camera, RenderOptions};
use vec3::{Point3, Vec3};
mod scene;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1080;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;

    let camera = Camera {
        origin: Point3::new(2.5, 1.0, 1.5),
        direction: Vec3::new(-0.7, -0.3, -1.0),
        focal_length: 1.5,
        image_width,
        image_height,
        viewport_width: 2.0,
    };

    let mut image = Image::new(image_width as u32, image_height as u32);

    let scene = scene::scene();

    camera.render(
        &scene,
        &RenderOptions {
            samples_per_pixel,
            bounce_limit: 10,
            progress_bar: true,
        },
        |(x, y), c| {
            image.set_pixel(x, y, c.into());
        },
    );

    image.save_png("./examples/balls/output-n.png");
}
