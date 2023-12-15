mod plane;
mod sphere;

pub enum Obj {
    Sphere(Sphere),
    Plane(Plane),
}

pub trait ToObj {
    fn to_obj(&self) -> Obj;
}

pub use plane::Plane;
pub use sphere::Sphere;
