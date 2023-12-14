use glam::Vec3A;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn grey(value: f32) -> Self {
        Self::new(value, value, value)
    }

    pub fn black() -> Self {
        Self::grey(0.0)
    }

    pub fn white() -> Self {
        Self::grey(1.0)
    }

    #[inline]
    pub(crate) fn to_vec3a(&self) -> Vec3A {
        Vec3A::new(self.r, self.g, self.b)
    }
}
