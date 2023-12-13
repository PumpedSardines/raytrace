use ray_trace::camera::Camera;
use ray_trace::shader::data::*;

use glam::f32::*;

use image::ImageBuffer;

fn main() {
    let aspect_ratio = 4.0 / 3.0;
    let image_width = 1024;
    let image_height = (image_width as f32 / aspect_ratio) as u32;
    let samples = 1;

    let mut output = vec![Vec3A::new(0.0, 0.0, 0.0); (image_width * image_height) as usize];

    for _ in 0..samples {
        let pass_output = ray_trace::shader::ray_trace::render(RenderData {
            camera: Camera {
                origin: Vec3A::new(0.0, 0.0, 0.0),
                direction: Vec3A::new(0.0, 0.0, -1.0),
                focal_length: 1.0,
                viewport_width: 2.0,
                image_width,
                image_height,
            },
            spheres: vec![
                Sphere {
                    origin: Vec3A::new(0.0, 0.0, -1.0),
                    radius: 0.5,
                    material: Material {
                        albedo: Vec3A::new(0.1, 0.2, 1.0),
                        roughness: 0.5,
                    },
                },
                Sphere {
                    origin: Vec3A::new(0.0, -100.5, 1.0),
                    radius: 100.0,
                    material: Material {
                        albedo: Vec3A::new(0.8, 0.8, 0.0),
                        roughness: 1.0,
                    },
                },
            ],
        });

        for i in 0..output.len() {
            output[i] += pass_output[i];
        }
    }

    for i in 0..output.len() {
        output[i] /= samples as f32;
    }

    let img = ImageBuffer::from_fn(image_width, image_height, |x, y| {
        let v = output[(x + y * image_width) as usize] * 255.0;
        let r = v.x as u8;
        let g = v.y as u8;
        let b = v.z as u8;

        image::Rgb([r, g, b])
    });

    img.save("examples/ball/output.png").unwrap();
}
