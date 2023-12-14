mod sphere;

pub enum Obj {
    Sphere(Sphere),
}

pub trait ToObj {
    fn to_obj(&self) -> Obj;
}

pub use sphere::Sphere;
