use crate::{
    gpu::ToTypeMapping,
    position::{Position, Vector},
};
use glam::f32::Vec3A;

/// Camera representation
pub struct Camera {
    /// The origin of the camera
    pub origin: Position,
    /// The direction the camera is facing
    pub direction: Vector,
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
    pub fn new() -> Self {
        Self {
            origin: Position::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
            focal_length: 1.0,
            viewport_width: 1.0,
            image_width: 1,
            image_height: 1,
        }
    }

    pub fn with_origin(mut self, origin: Position) -> Self {
        self.origin = origin;
        self
    }

    pub fn with_direction(mut self, direction: Vector) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_focal_length(mut self, focal_length: f32) -> Self {
        self.focal_length = focal_length;
        self
    }

    pub fn with_viewport_width(mut self, viewport_width: f32) -> Self {
        self.viewport_width = viewport_width;
        self
    }

    pub fn with_image_width(mut self, image_width: u32) -> Self {
        self.image_width = image_width;
        self
    }

    pub fn with_image_height(mut self, image_height: u32) -> Self {
        self.image_height = image_height;
        self
    }

    fn calc_viewport_upper_left(&self) -> Vec3A {
        let origin = self.origin.to_vec3a();
        let direction = self.direction.to_vec3a();

        origin - (self.calc_viewport_u() * 0.5) - (self.calc_viewport_v() * 0.5)
            + (direction.normalize() * self.focal_length)
    }

    fn calc_viewport_u(&self) -> Vec3A {
        let x = 1.0;
        let y = 0.0;
        let z = -1.0 * (x * self.direction.x()) / self.direction.z();

        Vec3A::new(x, y, z).normalize() * self.viewport_width
    }

    fn calc_viewport_v(&self) -> Vec3A {
        let viewport_height =
            (self.viewport_width / self.image_width as f32) * self.image_height as f32;

        let direction = self.direction.to_vec3a();

        self.calc_viewport_u().cross(direction).normalize() * -1.0 * viewport_height
    }

    fn calc_pixel_delta_u(&self) -> Vec3A {
        self.calc_viewport_u() / self.image_width as f32
    }

    fn calc_pixel_delta_v(&self) -> Vec3A {
        self.calc_viewport_v() / self.image_height as f32
    }
}

impl ToTypeMapping for Camera {
    type TypeMapping = crate::gpu::type_mapping::Camera;

    fn to_type_mapping(&self) -> Self::TypeMapping {
        Self::TypeMapping {
            origin: self.origin.to_vec3a(),
            viewport_upper_left: self.calc_viewport_upper_left(),
            pixel_delta_u: self.calc_pixel_delta_u(),
            pixel_delta_v: self.calc_pixel_delta_v(),
            image_width: self.image_width,
            image_height: self.image_height,
        }
    }
}
