#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Point3<T> = Vec3<T>;

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }
}

impl From<(f64, f64, f64)> for Vec3<f64> {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Vec3 { x, y, z }
    }
}

impl From<&(f64, f64, f64)> for Vec3<f64> {
    fn from((x, y, z): &(f64, f64, f64)) -> Self {
        Vec3 {
            x: *x,
            y: *y,
            z: *z,
        }
    }
}

impl<T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Copy> Vec3<T>
where
    f64: From<T>,
{
    pub fn normalized(&self) -> Vec3<f64> {
        let length = self.length();
        Vec3 {
            x: f64::from(self.x) / length,
            y: f64::from(self.y) / length,
            z: f64::from(self.z) / length,
        }
    }

    pub fn length(&self) -> f64 {
        f64::from(self.length_squared()).sqrt()
    }

    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Vec3<f64> {
    pub fn reflect(&self, normal: Vec3<f64>) -> Vec3<f64> {
        *self - normal.scalar(2.0 * self.dot(normal))
    }

    pub fn lerp(a: Vec3<f64>, b: Vec3<f64>, t: f64) -> Vec3<f64> {
        a.scalar(1.0 - t) + b.scalar(t)
    }

    pub fn zero() -> Vec3<f64> {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: std::ops::Sub<Output = T>> std::ops::Sub for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: std::ops::Neg> std::ops::Neg for Vec3<T> {
    type Output = Vec3<T::Output>;

    fn neg(self) -> Vec3<T::Output> {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: std::ops::Mul<Output = T>> std::ops::Mul for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T: std::ops::Mul<Output = T> + Copy> Vec3<T> {
    pub fn scalar(&self, scaling_factor: T) -> Vec3<T> {
        Vec3 {
            x: self.x * scaling_factor,
            y: self.y * scaling_factor,
            z: self.z * scaling_factor,
        }
    }
}

impl<T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Copy> Vec3<T> {
    pub fn dot(&self, v: Vec3<T>) -> T {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}

impl<T: std::ops::Add<Output = T> + Copy> Vec3<T> {
    pub fn add(&self, v: T) -> Vec3<T> {
        return Vec3 {
            x: self.x + v,
            y: self.y + v,
            z: self.z + v,
        };
    }
}

impl<T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + Copy> Vec3<T> {
    pub fn cross(&self, v: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }
}
