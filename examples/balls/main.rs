use image::ImageBuffer;
use ray_trace::{
    camera::Camera,
    color::Color,
    material::Material,
    objects::{Plane, Sphere},
    position::{Position, Vector},
    renderer::{RayTraceRenderOptions, RayTraceRenderer},
    world::World,
};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    let renderer = RayTraceRenderer::new();

    let world = World::new()
        .with_camera(Camera {
            origin: Position::new(2.5, 1.0, 1.5),
            direction: Vector::new(-0.7, -0.3, -1.0),
            focal_length: 1.5,
            viewport_width: 2.0,
            image_width,
            image_height,
        })
        .with_object(Sphere {
            origin: Position::new(-1.86, 0.1, -2.0),
            radius: 0.6,
            material: Material {
                albedo: Color::new(1.0, 0.0, 1.0),
                roughness: 0.6,
            },
        })
        .with_object(Sphere {
            origin: Position::new(-0.93, -0.2, -2.0),
            radius: 0.3,
            material: Material {
                albedo: Color::white(),
                roughness: 0.3,
            },
        })
        .with_object(Sphere {
            origin: Position::new(-0.31, -0.1, -1.5),
            radius: 0.4,
            material: Material {
                albedo: Color::new(0.1, 1.0, 0.1),
                roughness: 1.0,
            },
        })
        .with_object(Sphere {
            origin: Position::new(0.31, -0.2, -2.0),
            radius: 0.3,
            material: Material {
                albedo: Color::new(0.1, 0.1, 1.0),
                roughness: 0.6,
            },
        })
        .with_object(Sphere {
            origin: Position::new(0.63, -0.2, -1.0),
            radius: 0.3,
            material: Material {
                albedo: Color::new(1.0, 1.0, 0.2),
                roughness: 1.0,
            },
        })
        .with_object(Sphere {
            origin: Position::new(1.56, -0.2, -2.0),
            radius: 0.3,
            material: Material {
                albedo: Color::new(1.0, 0.2, 0.2),
                roughness: 1.0,
            },
        })
        .with_object(Plane {
            normal: Vector::new(0.0, 1.0, 0.0),
            distance: -0.5,
            material: Material {
                albedo: Color::new(0.2, 0.5, 1.0),
                roughness: 0.9,
            },
        });

    let options = RayTraceRenderOptions::default()
        .with_image_samples(10)
        .with_pixel_samples(200)
        .with_max_bounces(100);

    let colors = renderer.render(&world, &options);

    let img = ImageBuffer::from_fn(image_width, image_height, |x, y| {
        let v = colors[(x + y * image_width) as usize];
        let r = (v.r * 255.0) as u8;
        let g = (v.g * 255.0) as u8;
        let b = (v.b * 255.0) as u8;

        image::Rgb([r, g, b])
    });

    img.save("examples/balls/output.png").unwrap();
}
