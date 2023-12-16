use image::ImageBuffer;
use ray_trace::{
    camera::Camera,
    color::Color,
    material::Material,
    objects::{Sphere, Triangle},
    position::{Position, Vector},
    renderer::{RayTraceRenderOptions, RayTraceRenderer},
    world::World,
};

fn main() {
    let aspect_ratio = 4.0 / 3.0;
    let image_width = 800;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    let renderer = RayTraceRenderer::new();

    let world = World::new()
        .with_camera(Camera {
            origin: Position::new(0.0, 0.0, 1.0),
            direction: Vector::new(0.0, 0.0, -1.0),
            focal_length: 1.0,
            viewport_width: 2.0,
            image_width,
            image_height,
        })
        .with_objects(vec![
            Box::new(Triangle {
                points: [
                    Position::new(-0.25, -0.2, 0.0),
                    Position::new(0.0, 0.3, 0.0),
                    Position::new(0.25, -0.2, 0.0),
                ],
                material: Material {
                    albedo: Color::new(1.0, 0.0, 1.0),
                    roughness: 0.6,
                },
            }),
            Box::new(Sphere {
                origin: Position::new(0.0, 0.3, 0.0),
                radius: 0.03,
                material: Material {
                    albedo: Color::white(),
                    roughness: 1.0,
                },
            }),
            Box::new(Sphere {
                origin: Position::new(-0.25, -0.2, 0.0),
                radius: 0.03,
                material: Material {
                    albedo: Color::white(),
                    roughness: 1.0,
                },
            }),
            Box::new(Sphere {
                origin: Position::new(0.25, -0.2, 0.0),
                radius: 0.03,
                material: Material {
                    albedo: Color::white(),
                    roughness: 1.0,
                },
            }),
        ]);

    let options = RayTraceRenderOptions::default()
        .with_image_samples(10)
        .with_pixel_samples(200)
        .with_max_bounces(10);

    let colors = renderer.render(&world, &options);

    let img = ImageBuffer::from_fn(image_width, image_height, |x, y| {
        let v = colors[(x + y * image_width) as usize];
        let r = (v.r * 255.0) as u8;
        let g = (v.g * 255.0) as u8;
        let b = (v.b * 255.0) as u8;

        image::Rgb([r, g, b])
    });

    img.save("examples/triangle/output.png").unwrap();
}
