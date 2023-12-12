use ray_trace::{
    color::{Color, Gradient},
    object::{hittable, material::Material, mesh::Mesh, plane::Plane, sphere::Sphere},
    scene::Scene,
};
use vec3::Point3;

const MONKEY: &str = include_str!("./monkey.obj");

pub fn scene() -> Scene {
    let objects: Vec<Box<dyn hittable::Hittable>> = vec![
        Box::new(Plane::new(
            0.0,
            Point3::new(0.0, 1.0, 0.0),
            Material::new()
                .set_roughness(0.0)
                .set_albedo(Color::white()),
        )),
        Box::new(Mesh::from_obj(
            &obj::Obj::new(MONKEY),
            Material::new().set_albedo(Color::new(1.0, 0.0, 0.0)),
        )),
        Box::new(Sphere::new(
            Point3::new(1.0, 1.0, 240.0),
            100.0,
            Material::new().set_emission(Color::white() * Color::grey(10.0)),
        )),
    ];

    Scene {
        objects,
        sky_color: Color::black().into(),
    }
}
