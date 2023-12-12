use ray_trace::{
    color::{Color, Gradient},
    object::{hittable, material::Material, plane::Plane, triangle::Triangle},
    scene::Scene,
};
use vec3::Point3;

pub fn scene() -> Scene {
    let objects: Vec<Box<dyn hittable::Hittable>> = vec![Box::new(Triangle::new(
        [
            Point3::new(-0.25, 0.0, 0.0),
            Point3::new(0.0, 0.5, 0.0),
            Point3::new(0.25, 0.0, 0.0),
        ],
        Material::new()
            .set_albedo(Color::new(1.0, 0.0, 1.0))
            .set_roughness(0.6),
    ))];

    Scene {
        objects,
        sky_color: Gradient::new(vec![
            (0.0, Color::white()),
            (1.0, Color::new(0.5, 0.7, 1.0)),
        ]),
    }
}
