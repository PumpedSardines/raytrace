use image::ImageBuffer;
use ray_trace::{
    camera::Camera,
    color::Color,
    material::Material,
    objects::Sphere,
    position::{Position, Vector},
    renderer::{RayTraceRenderOptions, RayTraceRenderer},
    world::World,
};

fn main() {
    let aspect_ratio = 4.0 / 3.0;
    let image_width = 1920;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    let renderer = RayTraceRenderer::new();

    let mut world = World::new()
        .with_camera(Camera {
            origin: Position::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, -1.0),
            focal_length: 1.0,
            viewport_width: 2.0,
            image_width,
            image_height,
        })
        .with_objects(vec![
            Box::new(Sphere {
                origin: Position::new(0.0, 0.0, -1.0),
                radius: 0.5,
                material: Material {
                    albedo: Color::new(0.1, 0.2, 1.0),
                    roughness: 0.5,
                },
            }),
            Box::new(Sphere {
                origin: Position::new(0.0, -100.5, 1.0),
                radius: 100.0,
                material: Material {
                    albedo: Color::new(0.8, 0.8, 0.0),
                    roughness: 1.0,
                },
            }),
        ]);

    world.build();

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

    img.save("examples/ball/output.png").unwrap();
}
