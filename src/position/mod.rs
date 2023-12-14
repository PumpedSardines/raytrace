use glam::Vec3A;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    v: [f32; 3],
}

pub type Vector = Position;

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { v: [x, y, z] }
    }

    pub fn x(&self) -> f32 {
        self.v[0]
    }

    pub fn y(&self) -> f32 {
        self.v[1]
    }

    pub fn z(&self) -> f32 {
        self.v[2]
    }

    pub(crate) fn to_vec3a(&self) -> Vec3A {
        Vec3A::new(self.x(), self.y(), self.z())
    }
}
