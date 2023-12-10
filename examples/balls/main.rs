use image::Image;
use indicatif::ProgressBar;
use ray_trace::camera::{Camera, RenderOptions};
use vec3::{Point3, Vec3};
mod scene;

fn main() {
    let image_width = 1920;
    let image_height = 1080;
    let samples_per_pixel = 2000;

    let camera = Camera {
        origin: Point3::new(2.5, 1.0, 1.5),
        direction: Vec3::new(-0.7, -0.3, -1.0),
        focal_length: 1.5,
        image_width,
        image_height,
        viewport_width: 2.0,
    };

    let mut image = Image::new(image_width as u32, image_height as u32);

    let progress_bar = ProgressBar::new(image_height as u64);

    progress_bar.inc(1);

    let scene = scene::scene();

    camera.render(
        &scene,
        &RenderOptions {
            samples_per_pixel,
            bounce_limit: 50,
        },
        |(x, y), c| {
            image.set_pixel(x, y, c.into());
            if x == image_width - 1 {
                progress_bar.inc(1);
            }
        },
    );

    image.save_png("./examples/balls/output.png");
}
