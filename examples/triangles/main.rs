use image::{imageops::FilterType::Triangle, ImageBuffer};
use ray_trace::{
    camera::Camera,
    color::Color,
    material::Material,
    objects::{Mesh, Plane, Sphere},
    position::{Position, Vector},
    renderer::{RayTraceRenderOptions, RayTraceRenderer},
    world::World,
};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    let renderer = RayTraceRenderer::new();

    let mut mesh = Mesh::new(Material {
        albedo: Color::grey(0.8),
        roughness: 1.0,
    });

    let triangles = vec![
        [
            Position::new(-1.0, -0.2, 0.0),
            Position::new(-1.25, 0.3, 0.0),
            Position::new(-1.5, -0.2, 0.0),
        ],
        [
            Position::new(1.0, -0.2, 0.0),
            Position::new(1.25, 0.3, 0.0),
            Position::new(1.5, -0.2, 0.0),
        ],
        [
            Position::new(0.25, -0.2, 0.0),
            Position::new(0.0, 0.3, 0.0),
            Position::new(-0.25, -0.2, 0.0),
        ],
    ];

    let mut world = World::new().with_camera(Camera {
        origin: Position::new(0.0, 5.0, 8.0),
        direction: Vector::new(0.0, -0.5, -1.0),
        focal_length: 1.5,
        viewport_width: 2.0,
        image_width,
        image_height,
    });

    for triangle in triangles {
        mesh.add_triangle(triangle);

        for point in triangle {
            world.add_object(Sphere {
                origin: point,
                radius: 0.03,
                material: Material {
                    albedo: Color::white(),
                    roughness: 1.0,
                },
            });
        }
    }

    world.add_object(mesh);

    world.build();

    let options = RayTraceRenderOptions::default()
        .with_image_samples(10)
        .with_pixel_samples(100)
        .with_max_bounces(100);

    let colors = renderer.render(&world, &options);

    let img = ImageBuffer::from_fn(image_width, image_height, |x, y| {
        let v = colors[(x + y * image_width) as usize];
        let r = (v.r * 255.0) as u8;
        let g = (v.g * 255.0) as u8;
        let b = (v.b * 255.0) as u8;

        image::Rgb([r, g, b])
    });

    img.save("examples/triangles/output.png").unwrap();
}
