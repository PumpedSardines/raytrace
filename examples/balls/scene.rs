use ray_trace::{
    color::{Color, Gradient},
    object::{hittable, material::Material, plane::Plane, sphere::Sphere},
    scene::Scene,
};
use vec3::Point3;

pub fn scene() -> Scene {
    let objects: Vec<Box<dyn hittable::Hittable>> = vec![
        Box::new(Sphere::new(
            Point3::new(-1.86, 0.1, -2.0),
            0.6,
            Material::new()
                .set_albedo(Color::new(1.0, 0.0, 1.0))
                .set_roughness(0.6),
        )),
        Box::new(Sphere::new(
            Point3::new(-0.93, -0.2, -2.0),
            0.3,
            Material::new()
                .set_albedo(Color::white())
                .set_roughness(0.3),
        )),
        Box::new(Sphere::new(
            Point3::new(-0.31, -0.1, -1.5),
            0.4,
            Material::new().set_albedo(Color::new(0.1, 1.0, 0.1)),
        )),
        Box::new(Sphere::new(
            Point3::new(0.31, -0.2, -2.0),
            0.3,
            Material::new()
                .set_albedo(Color::new(0.1, 0.1, 1.0))
                .set_roughness(0.6),
        )),
        Box::new(Sphere::new(
            Point3::new(0.63, -0.2, -1.0),
            0.3,
            Material::new()
                .set_albedo(Color::new(1.0, 1.0, 0.2))
                .set_roughness(1.0),
        )),
        Box::new(Sphere::new(
            Point3::new(1.56, -0.2, -2.0),
            0.3,
            Material::new().set_albedo(Color::new(1.0, 0.2, 0.2)),
        )),
        Box::new(Plane::new(
            -0.5,
            Point3::new(0.0, 1.0, 0.0),
            Material::new()
                .set_roughness(0.9)
                .set_albedo(Color::new(0.2, 0.5, 1.0)),
        )),
    ];

    Scene {
        objects,
        sky_color: Gradient::new(vec![
            (0.0, Color::white()),
            (1.0, Color::new(0.5, 0.7, 1.0)),
        ]),
    }
}
