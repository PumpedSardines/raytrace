use crate::{
    object::{
        hittable::{HitRecord, Hittable},
        material::Material,
    },
    ray::Ray,
};
use vec3::{Point3, Vec3};

// Triangle intersection algorithm from:
// https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/ray-triangle-intersection-geometric-solution.html
//
// Triangle intesection is trickier than sphere intersection so there are a lot more pre computing
// steps here.

pub struct Triangle {
    pub material: Material,
    points: [Point3<f64>; 3],
    normal: Vec3<f64>,
    d: f64,
}

impl Triangle {
    pub fn new(points: [Point3<f64>; 3], material: Material) -> Triangle {
        let mut triangle = Triangle {
            normal: Vec3::zero(),
            points,
            material,
            d: 0.0,
        };
        triangle.set_points(points);
        triangle
    }

    pub fn set_points(&mut self, points: [Point3<f64>; 3]) {
        self.points = points;
        let v1 = points[1] - points[0];
        let v2 = points[2] - points[0];
        self.normal = v1.cross(v2).normalized();
        self.d = -self.normal.dot(points[0]);
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // If this is zero, the ray is parallel to the plane and no intersection occurs
        let normal_dot_direction = self.normal.dot(r.direction);

        if normal_dot_direction.abs() < 1e-6 {
            return None;
        }

        // First calculate where the ray hits the plane
        let t = -(self.normal.dot(r.origin) + self.d) / normal_dot_direction;

        // Out of bounds
        if t < t_min || t > t_max {
            return None;
        }

        // The point where the ray hits the plane
        let point = r.at(t);

        // I didn't really understand the math behind this part.
        // But here we basically check if the point lays inside the triangle on the plane.
        let edge0 = self.points[1] - self.points[0];
        let edge1 = self.points[2] - self.points[1];
        let edge2 = self.points[0] - self.points[2];
        let c0 = point - self.points[0];
        let c1 = point - self.points[1];
        let c2 = point - self.points[2];
        let test0 = edge0.cross(c0);
        let test1 = edge1.cross(c1);
        let test2 = edge2.cross(c2);
        if self.normal.dot(test0) < 0.0
            || self.normal.dot(test1) < 0.0
            || self.normal.dot(test2) < 0.0
        {
            return None;
        }

        Some(HitRecord {
            t,
            p: point,
            normal: self.normal,
            front_face: true,
            material: self.material,
        })
    }
}
