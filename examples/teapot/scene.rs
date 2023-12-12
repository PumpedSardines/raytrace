use ray_trace::{
    color::{Color, Gradient},
    object::{hittable, material::Material, mesh::Mesh, plane::Plane},
    scene::Scene,
};
use vec3::Point3;

const TEAPOT: &str = include_str!("./teapot.obj");

pub fn scene() -> Scene {
    let objects: Vec<Box<dyn hittable::Hittable>> = vec![
        Box::new(Plane::new(
            0.0,
            Point3::new(0.0, 1.0, 0.0),
            Material::new()
                .set_roughness(0.9)
                .set_albedo(Color::new(0.2, 0.5, 1.0)),
        )),
        Box::new(Mesh::from_obj(
            &obj::Obj::new(TEAPOT),
            Material::new().set_albedo(Color::grey(0.8)),
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
