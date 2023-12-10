use crate::{color::Gradient, object::hittable::Hittable};

pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
    pub sky_color: Gradient,
}
