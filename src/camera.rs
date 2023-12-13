use crate::shader::data::CameraBake;
use glam::f32::Vec3A;

/// Camera representation
pub struct Camera {
    /// The origin of the camera
    pub origin: Vec3A,
    /// The direction the camera is facing
    pub direction: Vec3A,
    /// The focal length of the camera
    pub focal_length: f32,
    /// How wide the image width is in viewport space
    pub viewport_width: f32,
    /// The width of the image in pixels
    pub image_width: u32,
    /// The height of the image in pixels
    pub image_height: u32,
}

impl Camera {
    fn calc_viewport_upper_left(&self) -> Vec3A {
        self.origin - self.calc_viewport_u() * 0.5 - self.calc_viewport_v() * 0.5
            + (self.direction.normalize() * self.focal_length)
    }

    fn calc_viewport_u(&self) -> Vec3A {
        let x = 1.0;
        let y = 0.0;
        let z = -1.0 * (x * self.direction.x) / self.direction.z;

        Vec3A::new(x, y, z).normalize() * self.viewport_width
    }

    fn calc_viewport_v(&self) -> Vec3A {
        let viewport_height =
            (self.viewport_width / self.image_width as f32) * self.image_height as f32;

        self.calc_viewport_u().cross(self.direction).normalize() * -1.0 * viewport_height
    }

    fn calc_pixel_delta_u(&self) -> Vec3A {
        self.calc_viewport_u() * (1.0 / self.image_width as f32)
    }

    fn calc_pixel_delta_v(&self) -> Vec3A {
        self.calc_viewport_v() * (1.0 / self.image_height as f32)
    }

    pub(crate) fn bake(&self) -> CameraBake {
        CameraBake {
            origin: self.origin,
            viewport_upper_left: self.calc_viewport_upper_left(),
            pixel_delta_u: self.calc_pixel_delta_u(),
            pixel_delta_v: self.calc_pixel_delta_v(),
            image_width: self.image_width,
            image_height: self.image_height,
        }
    }
}
