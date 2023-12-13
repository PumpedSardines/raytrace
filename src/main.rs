mod camera;
mod shader;

use camera::Camera;
use glam::f32::*;
use shader::data::*;

use image::ImageBuffer;

fn main() {
    let image_width = 1024;
    let image_height = 768;
    let samples = 1;

    let mut output = vec![Vec3A::new(0.0, 0.0, 0.0); (image_width * image_height) as usize];

    for _ in 0..samples {
        let pass_output = shader::ray_trace::render(RenderData {
            camera: Camera {
                origin: Vec3A::new(0.0, 0.0, 0.0),
                direction: Vec3A::new(0.0, 0.0, -1.0),
                focal_length: 1.0,
                viewport_width: 2.0,
                image_width: 1024,
                image_height: 768,
            },
            spheres: vec![
                Sphere {
                    origin: Vec3A::new(0.0, 0.0, -1.0),
                    radius: 0.5,
                },
                Sphere {
                    origin: Vec3A::new(0.0, -100.5, 1.0),
                    radius: 100.0,
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

    let img = ImageBuffer::from_fn(1024, 768, |x, y| {
        let v = output[(x + y * 1024) as usize] * 255.0;
        let r = v.x as u8;
        let g = v.y as u8;
        let b = v.z as u8;
        image::Rgb([r, g, b])
    });

    img.save("output.png").unwrap();
}
