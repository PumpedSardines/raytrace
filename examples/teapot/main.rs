use image::ImageBuffer;
use ray_trace::{
    camera::Camera,
    color::Color,
    material::Material,
    objects::{Mesh, Plane},
    position::{Position, Vector},
    renderer::{RayTraceRenderOptions, RayTraceRenderer},
    world::World,
};

const TEAPOT_OBJ: &str = include_str!("teapot.obj");

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    let renderer = RayTraceRenderer::new();

    let mut world = World::new()
        .with_camera(Camera {
            origin: Position::new(0.0, 5.0, 8.0),
            direction: Vector::new(0.0, -0.5, -1.0),
            focal_length: 1.5,
            viewport_width: 2.0,
            image_width,
            image_height,
        })
        .with_objects(vec![
            Box::new(Plane {
                normal: Vector::new(0.0, 1.0, 0.0),
                distance: 0.0,
                material: Material {
                    albedo: Color::new(0.2, 0.5, 1.0),
                    roughness: 0.9,
                },
            }),
            Box::new(Mesh::from_obj(
                TEAPOT_OBJ,
                Material {
                    albedo: Color::grey(0.8),
                    roughness: 1.0,
                },
            )),
        ]);

    world.build();

    let options = RayTraceRenderOptions::default()
        .with_image_samples(100)
        .with_pixel_samples(1)
        .with_max_bounces(100);

    let colors = renderer.render(&world, &options);

    let img = ImageBuffer::from_fn(image_width, image_height, |x, y| {
        let v = colors[(x + y * image_width) as usize];
        let r = (v.r * 255.0) as u8;
        let g = (v.g * 255.0) as u8;
        let b = (v.b * 255.0) as u8;

        image::Rgb([r, g, b])
    });

    img.save("examples/teapot/output.png").unwrap();
}
