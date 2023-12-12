use ray_trace::{
    color::{Color, Gradient},
    object::{hittable, material::Material, plane::Plane, triangle::Triangle},
    scene::Scene,
};
use vec3::Point3;

const TEAPOT: &str = include_str!("./teapot.obj");

pub fn scene() -> Scene {
    let mut objects: Vec<Box<dyn hittable::Hittable>> = vec![Box::new(Plane::new(
        0.0,
        Point3::new(0.0, 1.0, 0.0),
        Material::new()
            .set_roughness(0.9)
            .set_albedo(Color::new(0.2, 0.5, 1.0)),
    ))];

    let mut vertexes = vec![];

    for line in TEAPOT.to_string().lines() {
        if line.starts_with("v ") {
            let mut coords = line.split_whitespace().skip(1);
            let x = coords.next().unwrap().parse::<f64>().unwrap();
            let y = coords.next().unwrap().parse::<f64>().unwrap();
            let z = coords.next().unwrap().parse::<f64>().unwrap();

            vertexes.push(Point3::new(x, y, z));
        }

        if line.starts_with("f ") {
            let mut coords = line.split_whitespace().skip(1);
            let point0 = coords.next().unwrap().parse::<usize>().unwrap();
            let point1 = coords.next().unwrap().parse::<usize>().unwrap();
            let point2 = coords.next().unwrap().parse::<usize>().unwrap();

            objects.push(Box::new(Triangle::new(
                [
                    vertexes[point0 - 1],
                    vertexes[point1 - 1],
                    vertexes[point2 - 1],
                ],
                Material::new().set_albedo(Color::grey(0.8)),
            )));
        }
    }

    Scene {
        objects,
        sky_color: Gradient::new(vec![
            (0.0, Color::white()),
            (1.0, Color::new(0.5, 0.7, 1.0)),
        ]),
    }
}
